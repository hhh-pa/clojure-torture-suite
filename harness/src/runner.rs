//! Corpus runner: discover `torture/**/*.clj`, parse directives, execute on
//! backends, judge, and report DejaGNU-style results.

use crate::directives::{self, Directives, DoWhat, Mode};
use clap::Args;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Args)]
pub struct RunArgs {
    /// Only run tests whose path contains one of these
    #[arg(value_name = "PATTERN")]
    pub patterns: Vec<String>,
    /// Backend(s) to run: jvm, clojure-rs, all (repeatable)
    #[arg(long, action = clap::ArgAction::Append)]
    pub backend: Vec<String>,
    /// Torture corpus directory
    #[arg(long, default_value = "torture")]
    pub torture_dir: PathBuf,
    /// Directory with clojure/spec/core.specs jars (JVM backend)
    #[arg(long, default_value = "harness/lib")]
    pub jars: PathBuf,
    /// clojure-rs binary (clojure-rs backend)
    #[arg(long, default_value = "clojure-rs")]
    pub clojure_rs_bin: String,
    /// Runtime archive for `clojure-rs build`
    #[arg(long)]
    pub runtime_archive: Option<PathBuf>,
    /// Parallel jobs
    #[arg(long, short, default_value_t = 8)]
    pub jobs: usize,
    /// Default per-test timeout in seconds (dg-timeout wins)
    #[arg(long, default_value_t = 30)]
    pub timeout: u64,
    /// Require byte-identical stdout across backends
    #[arg(long)]
    pub diff_backends: bool,
    /// Unexpected passes fail the run
    #[arg(long)]
    pub strict_xpass: bool,
    /// DejaGNU-style summary file to write
    #[arg(long, default_value = "results.sum")]
    pub sum_file: PathBuf,
    /// Keep AOT temp dirs and read wrappers (debugging)
    #[arg(long)]
    pub keep_tmp: bool,
    /// List selected tests and exit
    #[arg(long)]
    pub list: bool,
    /// Verbose: print commands and outputs
    #[arg(long, short)]
    pub verbose: bool,
}

#[derive(Debug, Clone)]
struct Test {
    path: PathBuf,
    rel: String,
    directives: Directives,
}

#[derive(Debug, Clone)]
struct Detail {
    cmd: Vec<String>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    note: Option<String>,
}

#[derive(Debug, Clone)]
struct JobResult {
    status: &'static str,
    label: String,
    rel: String,
    backend: String,
    instance: String,
    reason: String,
    detail: Detail,
}

impl JobResult {
    fn unsupported(rel: &str, backend: &str, instance: &str, note: &str) -> Self {
        JobResult {
            status: "UNSUPPORTED",
            label: format!("{rel} ({backend}/{instance})"),
            rel: rel.to_string(),
            backend: backend.to_string(),
            instance: instance.to_string(),
            reason: String::new(),
            detail: Detail {
                cmd: Vec::new(),
                stdout: Vec::new(),
                stderr: Vec::new(),
                note: Some(note.to_string()),
            },
        }
    }
}

fn clj_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn truncate_debug(bytes: &[u8], max: usize) -> String {
    let s = lossy(bytes);
    if s.len() > max {
        format!("{:?}...<truncated>", &s[..max])
    } else {
        format!("{s:?}")
    }
}

/// Outcome of one backend invocation.
#[derive(Debug, Clone)]
pub struct Outcome {
    pub cmd: Vec<String>,
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub timed_out: bool,
}

impl Outcome {
    fn new(
        cmd: Vec<String>,
        code: Option<i32>,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
        timed_out: bool,
    ) -> Self {
        Outcome {
            cmd,
            code,
            stdout,
            stderr,
            timed_out,
        }
    }
}

/// Backend executors shared by `run` and `fuzz`.
pub struct Backends {
    pub jvm_classpath: Option<String>,
    pub cljr_bin: Option<String>,
    pub runtime_archive: Option<PathBuf>,
    pub keep_tmp: bool,
    tmp_counter: AtomicU64,
}

impl Backends {
    pub fn new(
        jvm_classpath: Option<String>,
        cljr_bin: Option<String>,
        runtime_archive: Option<PathBuf>,
        keep_tmp: bool,
    ) -> Self {
        Backends {
            jvm_classpath,
            cljr_bin,
            runtime_archive,
            keep_tmp,
            tmp_counter: AtomicU64::new(0),
        }
    }

    fn jvm_base(&self) -> Vec<String> {
        vec![
            "java".into(),
            "-Duser.language=en".into(),
            "-Duser.country=US".into(),
            "-Duser.timezone=UTC".into(),
            "-Dfile.encoding=UTF-8".into(),
            "-cp".into(),
            self.jvm_classpath.clone().unwrap(),
            "clojure.main".into(),
        ]
    }

    /// JVM script mode: `clojure.main <file> <args...>`.
    pub fn run_jvm_script(&self, file: &Path, args: &[String], timeout: Duration) -> Outcome {
        let mut cmd = self.jvm_base();
        cmd.push(abs_str(file));
        cmd.extend(args.iter().cloned());
        let (code, stdout, stderr, timed_out) = run_cmd(&cmd, timeout);
        Outcome::new(cmd, code, stdout, stderr, timed_out)
    }

    /// JVM eval mode: `clojure.main -e <expr>` (for load/read wrappers).
    pub fn run_jvm_eval(&self, expr: &str, timeout: Duration) -> Outcome {
        let mut cmd = self.jvm_base();
        cmd.push("-e".into());
        cmd.push(expr.to_string());
        let (code, stdout, stderr, timed_out) = run_cmd(&cmd, timeout);
        Outcome::new(cmd, code, stdout, stderr, timed_out)
    }

    /// `clojure-rs run <file> [-- <args...>]`.
    pub fn run_cljr_script(&self, file: &Path, args: &[String], timeout: Duration) -> Outcome {
        let mut cmd = vec![self.cljr_bin.clone().unwrap(), "run".into(), abs_str(file)];
        if !args.is_empty() {
            cmd.push("--".into());
            cmd.extend(args.iter().cloned());
        }
        let (code, stdout, stderr, timed_out) = run_cmd(&cmd, timeout);
        Outcome::new(cmd, code, stdout, stderr, timed_out)
    }

    /// `clojure-rs build <file> -o <tmpbin>` then execute it.
    /// Returns (build outcome, run outcome or None when the build failed).
    pub fn run_cljr_aot(&self, file: &Path, timeout: Duration) -> (Outcome, Option<Outcome>) {
        let id = self.tmp_counter.fetch_add(1, Ordering::SeqCst);
        let tmpdir = std::env::temp_dir().join(format!("torture-aot-{}-{id}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmpdir);
        let binary = tmpdir.join("torture-bin");
        let mut build_cmd = vec![
            self.cljr_bin.clone().unwrap(),
            "build".into(),
            abs_str(file),
            "-o".into(),
            binary.to_string_lossy().into_owned(),
        ];
        if let Some(ra) = &self.runtime_archive {
            build_cmd.push("--runtime-archive".into());
            build_cmd.push(ra.to_string_lossy().into_owned());
        }
        let (code, stdout, stderr, timed_out) = run_cmd(&build_cmd, timeout);
        let build = Outcome::new(build_cmd, code, stdout, stderr, timed_out);
        let run = if build.code == Some(0) && !build.timed_out {
            let bin_cmd = vec![binary.to_string_lossy().into_owned()];
            let (code, stdout, stderr, timed_out) = run_cmd(&bin_cmd, timeout);
            Some(Outcome::new(bin_cmd, code, stdout, stderr, timed_out))
        } else {
            None
        };
        if !self.keep_tmp {
            let _ = std::fs::remove_dir_all(&tmpdir);
        }
        (build, run)
    }
}

/// Run a command with a timeout. Returns (exit code, stdout, stderr, timed out).
fn run_cmd(argv: &[String], timeout: Duration) -> (Option<i32>, Vec<u8>, Vec<u8>, bool) {
    let mut child = match Command::new(&argv[0])
        .args(&argv[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return (None, Vec::new(), e.to_string().into_bytes(), false),
    };
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = read_all(child.stdout.take());
                let err = read_all(child.stderr.take());
                return (status.code(), out, err, false);
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let out = read_all(child.stdout.take());
                    let err = read_all(child.stderr.take());
                    return (None, out, err, true);
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(e) => return (None, Vec::new(), e.to_string().into_bytes(), false),
        }
    }
}

fn read_all<R: std::io::Read>(pipe: Option<R>) -> Vec<u8> {
    let mut buf = Vec::new();
    if let Some(mut p) = pipe {
        let _ = p.read_to_end(&mut buf);
    }
    buf
}

pub(crate) fn which(name: &str) -> Option<PathBuf> {
    if name.contains('/') && Path::new(name).is_file() {
        return Some(PathBuf::from(name));
    }
    if Path::new(name).is_file() {
        return Some(PathBuf::from(name));
    }
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join(name))
            .find(|p| is_executable(p))
    })
}

fn is_executable(p: &Path) -> bool {
    if !p.is_file() {
        return false;
    }
    p.metadata()
        .map(|m| {
            use std::os::unix::fs::PermissionsExt;
            m.permissions().mode() & 0o111 != 0
        })
        .unwrap_or(false)
}

fn abs_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| p.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

fn read_expr(test_path: &Path, jvm: bool) -> String {
    let abs = abs_str(test_path);
    let read = if jvm {
        // Plain read-string rejects #?(...) outside .cljc; the port's
        // reader accepts conditionals by default, so only the JVM
        // needs the opt-in (same accepted language either way).
        format!(
            "(read-string {{:read-cond :allow}} (slurp {}))",
            clj_string(&abs)
        )
    } else {
        format!("(read-string (slurp {}))", clj_string(&abs))
    };
    format!("(do {read} (println \"READ-OK\"))")
}

struct Harness {
    opts: RunArgs,
    backends: Backends,
}

impl Harness {
    fn write_wrapper(&self, test: &Test) -> String {
        let safe: String = test
            .rel
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let path = std::env::temp_dir().join(format!("torture-read-{safe}.clj"));
        let content = format!(
            ";; generated read wrapper for {}\n{}\n",
            test.path.display(),
            read_expr(&test.path, false)
        );
        let _ = std::fs::write(&path, content);
        path.to_string_lossy().into_owned()
    }

    fn execute(&self, test: &Test, backend: &str, instance: &str) -> JobResult {
        let rel = &test.rel;
        if test.directives.skip.iter().any(|b| b == backend) {
            return JobResult::unsupported(rel, backend, instance, "dg-skip-backend");
        }
        if backend == "jvm" && self.backends.jvm_classpath.is_none() {
            return JobResult::unsupported(
                rel,
                backend,
                instance,
                "jars missing (see scripts/fetch-clojure-jars.sh)",
            );
        }
        if backend == "clojure-rs" && self.backends.cljr_bin.is_none() {
            return JobResult::unsupported(rel, backend, instance, "clojure-rs binary not found");
        }
        let timeout = Duration::from_secs(test.directives.timeout.unwrap_or(self.opts.timeout));
        if instance == "aot" {
            return self.execute_aot(test, timeout);
        }
        let d = &test.directives;
        let outcome = match (backend, d.do_what) {
            ("jvm", DoWhat::Run | DoWhat::FailRun) => {
                self.backends.run_jvm_script(&test.path, &d.args, timeout)
            }
            ("jvm", DoWhat::Load | DoWhat::FailLoad) => self.backends.run_jvm_eval(
                &format!("(load-file {})", clj_string(&abs_str(&test.path))),
                timeout,
            ),
            ("jvm", DoWhat::Read | DoWhat::FailRead) => self
                .backends
                .run_jvm_eval(&read_expr(&test.path, true), timeout),
            (_, DoWhat::Read | DoWhat::FailRead) => {
                let wrapper = self.write_wrapper(test);
                self.backends
                    .run_cljr_script(Path::new(&wrapper), &[], timeout)
            }
            (_, _) => self.backends.run_cljr_script(&test.path, &d.args, timeout),
        };
        self.judge(
            test,
            backend,
            instance,
            outcome.cmd.clone(),
            outcome.code,
            outcome.stdout,
            outcome.stderr,
            outcome.timed_out,
            timeout,
            None,
            false,
        )
    }

    fn execute_aot(&self, test: &Test, timeout: Duration) -> JobResult {
        let (build, run) = self.backends.run_cljr_aot(&test.path, timeout);
        if build.timed_out || build.code != Some(0) {
            let note = if build.timed_out {
                "timeout building".to_string()
            } else {
                format!(
                    "build exit {}",
                    build
                        .code
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "?".into())
                )
            };
            let forced = if build.code == Some(0) {
                build.code
            } else {
                Some(1)
            };
            return self.judge(
                test,
                "clojure-rs",
                "aot",
                build.cmd.clone(),
                forced,
                build.stdout,
                build.stderr,
                build.timed_out,
                timeout,
                Some(note),
                true,
            );
        }
        let run = run.unwrap();
        self.judge(
            test,
            "clojure-rs",
            "aot",
            run.cmd.clone(),
            run.code,
            run.stdout,
            run.stderr,
            run.timed_out,
            timeout,
            None,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn judge(
        &self,
        test: &Test,
        backend: &str,
        instance: &str,
        cmd: Vec<String>,
        code: Option<i32>,
        out: Vec<u8>,
        err: Vec<u8>,
        timed_out: bool,
        timeout: Duration,
        note: Option<String>,
        force_fail: bool,
    ) -> JobResult {
        let label = format!("{} ({backend}/{instance})", test.rel);
        let detail = Detail {
            cmd,
            stdout: out.clone(),
            stderr: err.clone(),
            note,
        };
        let d = &test.directives;
        let fail = |reason: String| {
            self.verdict(
                test,
                backend,
                instance,
                &label,
                false,
                reason,
                detail.clone(),
            )
        };
        if timed_out {
            return fail(format!("timeout after {}s", timeout.as_secs()));
        }
        if force_fail || code.is_none() {
            let reason = detail
                .note
                .clone()
                .unwrap_or_else(|| "could not execute".into());
            return fail(reason);
        }
        let code = code.unwrap();
        if d.do_what.expect_zero() && code != 0 {
            return fail(format!("exit {code}, want 0"));
        }
        if !d.do_what.expect_zero() && code == 0 {
            return fail("exit 0, want nonzero".into());
        }
        if !d.do_what.expect_zero() {
            if let Some(re) = &d.error_re {
                if !regex_match(re, &lossy(&err)) {
                    return fail(format!("stderr lacks /{re}/"));
                }
            }
        }
        if d.do_what.expect_zero() {
            if let Some(want) = &d.output {
                if &out != want {
                    return fail(format!(
                        "stdout {}, want {}",
                        truncate_debug(&out, 200),
                        truncate_debug(want, 200)
                    ));
                }
            }
            if let Some(re) = &d.output_re {
                if !regex_match(re, &lossy(&out)) {
                    return fail(format!("stdout lacks /{re}/"));
                }
            }
        }
        self.verdict(test, backend, instance, &label, true, "ok".into(), detail)
    }

    #[allow(clippy::too_many_arguments)]
    fn verdict(
        &self,
        test: &Test,
        backend: &str,
        instance: &str,
        label: &str,
        passed: bool,
        reason: String,
        detail: Detail,
    ) -> JobResult {
        let xfailed = test.directives.xfail.iter().any(|b| b == backend);
        let status = match (passed, xfailed) {
            (true, true) => "XPASS",
            (true, false) => "PASS",
            (false, true) => "XFAIL",
            (false, false) => "FAIL",
        };
        JobResult {
            status,
            label: label.to_string(),
            rel: test.rel.clone(),
            backend: backend.to_string(),
            instance: instance.to_string(),
            reason,
            detail,
        }
    }

    fn report(&self, r: &JobResult, print_lock: &Mutex<()>) {
        let _guard = print_lock.lock().unwrap();
        println!("{}: {}", r.status, r.label);
        if r.status == "FAIL"
            || r.status == "XPASS"
            || (self.opts.verbose && r.status != "UNSUPPORTED")
        {
            if !r.detail.cmd.is_empty() {
                println!("    cmd: {}", r.detail.cmd.join(" "));
            }
            if !r.reason.is_empty() {
                println!("    reason: {}", r.reason);
            }
            if !r.detail.stdout.is_empty() {
                println!("    stdout: {}", truncate_debug(&r.detail.stdout, 2000));
            }
            if !r.detail.stderr.is_empty() {
                println!("    stderr: {}", truncate_debug(&r.detail.stderr, 2000));
            }
        }
    }
}

fn regex_match(pattern: &str, text: &str) -> bool {
    match Regex::new(pattern) {
        Ok(re) => re.is_match(text),
        Err(_) => false,
    }
}

fn discover(torture_dir: &Path, patterns: &[String]) -> (Vec<Test>, Vec<String>) {
    let mut files = Vec::new();
    let mut stack = vec![torture_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let mut names: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        names.sort();
        for path in names {
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().map(|e| e == "clj").unwrap_or(false) {
                files.push(path);
            }
        }
    }
    files.sort();
    let mut tests = Vec::new();
    let mut errors = Vec::new();
    for path in files {
        let rel = path
            .strip_prefix(torture_dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if !patterns.is_empty() && !patterns.iter().any(|p| rel.contains(p)) {
            continue;
        }
        match directives::parse_file(&path) {
            Ok(d) => tests.push(Test {
                path,
                rel,
                directives: d,
            }),
            Err(e) => errors.push(format!("{rel}: {e}")),
        }
    }
    (tests, errors)
}

fn check_diff(results: &[JobResult], tests: &[Test]) -> Vec<(String, Vec<String>)> {
    use std::collections::BTreeMap;
    let do_of: BTreeMap<&str, DoWhat> = tests
        .iter()
        .map(|t| (t.rel.as_str(), t.directives.do_what))
        .collect();
    let mut by_test: BTreeMap<&str, Vec<&JobResult>> = BTreeMap::new();
    for r in results {
        if r.status == "UNSUPPORTED" {
            continue;
        }
        by_test.entry(r.rel.as_str()).or_default().push(r);
    }
    let mut problems = Vec::new();
    for (rel, group) in &by_test {
        let do_what = do_of.get(rel).copied().unwrap_or(DoWhat::Run);
        if !do_what.expect_zero() {
            continue;
        }
        let mut outs: BTreeMap<Vec<u8>, Vec<String>> = BTreeMap::new();
        for r in group {
            if r.status == "PASS" || r.status == "XPASS" {
                outs.entry(r.detail.stdout.clone())
                    .or_default()
                    .push(format!("{}/{}", r.backend, r.instance));
            }
        }
        if outs.len() > 1 {
            let mut names: Vec<String> = outs.into_values().flatten().collect();
            names.sort();
            problems.push((rel.to_string(), names));
        }
    }
    problems
}

pub fn run(opts: RunArgs) -> i32 {
    let mut backends: Vec<String> = Vec::new();
    for b in &opts.backend {
        if b == "all" {
            backends.extend(["jvm".to_string(), "clojure-rs".to_string()]);
        } else if b == "jvm" || b == "clojure-rs" {
            backends.push(b.clone());
        } else {
            eprintln!("bad --backend '{b}' (want jvm, clojure-rs, all)");
            return 2;
        }
    }
    if backends.is_empty() {
        backends.push("jvm".to_string());
    }
    backends.dedup();

    let (tests, errors) = discover(&opts.torture_dir, &opts.patterns);
    if opts.list {
        for t in &tests {
            println!("{}", t.rel);
        }
        return 0;
    }
    if tests.is_empty() && errors.is_empty() {
        eprintln!("no tests selected");
        return 2;
    }

    let jvm_classpath = if backends.contains(&"jvm".to_string()) {
        find_jars(&opts.jars)
    } else {
        None
    };
    let cljr_bin = if backends.contains(&"clojure-rs".to_string()) {
        which(&opts.clojure_rs_bin).map(|p| p.to_string_lossy().into_owned())
    } else {
        None
    };
    if backends.contains(&"jvm".to_string()) && jvm_classpath.is_none() {
        eprintln!(
            "warning: JVM jars missing in {} (run scripts/fetch-clojure-jars.sh); jvm tests will be UNSUPPORTED",
            opts.jars.display()
        );
    }
    if backends.contains(&"clojure-rs".to_string()) && cljr_bin.is_none() {
        eprintln!(
            "warning: clojure-rs binary '{}' not found; clojure-rs tests will be UNSUPPORTED",
            opts.clojure_rs_bin
        );
    }

    let harness = Arc::new(Harness {
        opts: RunArgs {
            patterns: Vec::new(),
            backend: Vec::new(),
            torture_dir: opts.torture_dir.clone(),
            jars: opts.jars.clone(),
            clojure_rs_bin: opts.clojure_rs_bin.clone(),
            runtime_archive: opts.runtime_archive.clone(),
            jobs: opts.jobs,
            timeout: opts.timeout,
            diff_backends: opts.diff_backends,
            strict_xpass: opts.strict_xpass,
            sum_file: opts.sum_file.clone(),
            keep_tmp: opts.keep_tmp,
            list: false,
            verbose: opts.verbose,
        },
        backends: Backends::new(
            jvm_classpath,
            cljr_bin,
            opts.runtime_archive.clone(),
            opts.keep_tmp,
        ),
    });

    // Plan jobs.
    let mut jobs: Vec<(Test, String, String)> = Vec::new();
    for t in &tests {
        for b in &backends {
            if b == "clojure-rs" {
                match t.directives.mode {
                    Mode::Interp => jobs.push((t.clone(), b.clone(), "interp".into())),
                    Mode::Aot => jobs.push((t.clone(), b.clone(), "aot".into())),
                    Mode::InterpAot => {
                        jobs.push((t.clone(), b.clone(), "interp".into()));
                        jobs.push((t.clone(), b.clone(), "aot".into()));
                    }
                }
            } else {
                jobs.push((t.clone(), b.clone(), "interp".into()));
            }
        }
    }

    let print_lock = Arc::new(Mutex::new(()));
    let results: Arc<Mutex<Vec<JobResult>>> = Arc::new(Mutex::new(Vec::new()));
    let jobs = Arc::new(jobs);
    let n_threads = opts.jobs.max(1).min(jobs.len().max(1));
    let mut handles = Vec::new();
    for tid in 0..n_threads {
        let harness = Arc::clone(&harness);
        let print_lock = Arc::clone(&print_lock);
        let results = Arc::clone(&results);
        let jobs = Arc::clone(&jobs);
        handles.push(std::thread::spawn(move || {
            let mut i = tid;
            while i < jobs.len() {
                let (t, b, inst) = &jobs[i];
                let r = harness.execute(t, b, inst);
                harness.report(&r, &print_lock);
                results.lock().unwrap().push(r);
                i += n_threads;
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    let mut results = results.lock().unwrap().clone();
    for e in &errors {
        let r = JobResult {
            status: "ERROR",
            label: e.clone(),
            rel: e.clone(),
            backend: "-".into(),
            instance: "-".into(),
            reason: String::new(),
            detail: Detail {
                cmd: Vec::new(),
                stdout: Vec::new(),
                stderr: Vec::new(),
                note: None,
            },
        };
        harness.report(&r, &print_lock);
        results.push(r);
    }

    let mut diff_problems = Vec::new();
    if opts.diff_backends && backends.len() > 1 {
        diff_problems = check_diff(&results, &tests);
        let _guard = print_lock.lock().unwrap();
        for (rel, names) in &diff_problems {
            println!("DFAIL: {rel} outputs differ: {names:?}");
        }
    }

    let mut counts = std::collections::BTreeMap::from([
        ("PASS", 0),
        ("FAIL", 0),
        ("XFAIL", 0),
        ("XPASS", 0),
        ("UNSUPPORTED", 0),
        ("ERROR", 0),
    ]);
    for r in &results {
        *counts.entry(r.status).or_insert(0) += 1;
    }
    println!("# of expected passes\t\t{}", counts["PASS"]);
    println!("# of unexpected failures\t{}", counts["FAIL"]);
    println!("# of expected failures\t\t{}", counts["XFAIL"]);
    println!("# of unexpected passes\t\t{}", counts["XPASS"]);
    println!("# of unsupported tests\t\t{}", counts["UNSUPPORTED"]);
    println!("# of unresolved testcases\t{}", counts["ERROR"]);
    if !diff_problems.is_empty() {
        println!("# of differential mismatches\t{}", diff_problems.len());
    }
    results.sort_by(|a, b| a.label.cmp(&b.label));
    let mut sum = String::new();
    for r in &results {
        if r.reason.is_empty() {
            sum.push_str(&format!("{}: {}\n", r.status, r.label));
        } else {
            sum.push_str(&format!("{}: {} ({})\n", r.status, r.label, r.reason));
        }
    }
    let _ = std::fs::write(&opts.sum_file, sum);

    if counts["FAIL"] > 0 || counts["ERROR"] > 0 || !diff_problems.is_empty() {
        return 1;
    }
    if opts.strict_xpass && counts["XPASS"] > 0 {
        return 1;
    }
    0
}

pub(crate) fn find_jars(jars_dir: &Path) -> Option<String> {
    let entries = std::fs::read_dir(jars_dir).ok()?;
    let mut has_clojure = false;
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".jar") && name.starts_with("clojure-") {
            has_clojure = true;
            break;
        }
    }
    if !has_clojure {
        return None;
    }
    let abs = jars_dir
        .canonicalize()
        .unwrap_or_else(|_| jars_dir.to_path_buf());
    Some(format!("{}/*", abs.display()))
}
