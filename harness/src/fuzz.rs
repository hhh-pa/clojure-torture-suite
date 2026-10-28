//! Differential fuzz driver: generate programs, run every backend, compare.
//!
//! Each seed must exit 0 with byte-identical stdout on all of:
//! - `interp` mode: JVM oracle + `clojure-rs run`;
//! - `aot` mode: JVM oracle + `clojure-rs run` + `clojure-rs build` binary.
//!
//! A mismatch is a compiler finding (or a generator bug — see the safety
//! contract in [`crate::gen`]); either way the seed, program, and all
//! backend outputs are kept under `--out` with a repro line.

use crate::gen::{self, GenMode};
use crate::runner::{Backends, Outcome};
use clap::Args;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Args)]
pub struct FuzzArgs {
    /// Program shape: interp or aot
    #[arg(long, default_value = "interp")]
    pub mode: String,
    /// First seed to fuzz
    #[arg(long, default_value_t = 1)]
    pub start_seed: u64,
    /// How many seeds to fuzz
    #[arg(long, default_value_t = 100)]
    pub count: u64,
    /// Max expression nesting depth
    #[arg(long, default_value_t = 4)]
    pub depth: u32,
    /// Directory for programs and failure artifacts
    #[arg(long, default_value = "fuzz-out")]
    pub out: PathBuf,
    /// Directory with clojure/spec/core.specs jars (JVM backend)
    #[arg(long, default_value = "harness/lib")]
    pub jars: PathBuf,
    /// clojure-rs binary (device under test)
    #[arg(long, default_value = "clojure-rs")]
    pub clojure_rs_bin: String,
    /// Runtime archive for `clojure-rs build`
    #[arg(long)]
    pub runtime_archive: Option<PathBuf>,
    /// Parallel jobs
    #[arg(long, short, default_value_t = 8)]
    pub jobs: usize,
    /// Per-program timeout in seconds
    #[arg(long, default_value_t = 30)]
    pub timeout: u64,
    /// Keep passing programs too (default: keep failures only)
    #[arg(long)]
    pub keep_pass: bool,
    /// Seeds expected to fail (known upstream findings), e.g. "2,10,11"
    #[arg(long, default_value = "")]
    pub xfail_seeds: String,
    /// Fail the run if an expected-failure seed passes (forces unpinning)
    #[arg(long)]
    pub strict_xpass: bool,
}

/// Parse a comma/whitespace-separated seed list like "2, 10,11".
pub fn parse_seed_list(s: &str) -> Result<std::collections::HashSet<u64>, String> {
    let mut out = std::collections::HashSet::new();
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(out);
    }
    for tok in trimmed.split(|c: char| c == ',' || c.is_whitespace()) {
        if tok.is_empty() {
            continue;
        }
        match tok.parse::<u64>() {
            Ok(n) => {
                out.insert(n);
            }
            Err(_) => return Err(format!("bad seed '{tok}' in --xfail-seeds (want u64 list)")),
        }
    }
    Ok(out)
}

struct SeedResult {
    passed: bool,
    detail: String,
}

/// DejaGNU-style verdict for one fuzzed seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Pass,
    XPass,
    XFail,
    Fail,
}

fn classify_seed(passed: bool, expected_failure: bool) -> Verdict {
    match (passed, expected_failure) {
        (true, false) => Verdict::Pass,
        (true, true) => Verdict::XPass,
        (false, true) => Verdict::XFail,
        (false, false) => Verdict::Fail,
    }
}

fn fuzz_exit_code(n_failed: usize, n_xpass: usize, strict_xpass: bool) -> i32 {
    if n_failed == 0 && (n_xpass == 0 || !strict_xpass) {
        0
    } else {
        1
    }
}

fn write_artifact(out_dir: &Path, seed: u64, suffix: &str, bytes: &[u8]) {
    let _ = std::fs::write(out_dir.join(format!("seed-{seed}.{suffix}")), bytes);
}

fn save_outcomes(out_dir: &Path, seed: u64, stages: &[(&str, &Outcome)]) {
    for (name, o) in stages {
        write_artifact(out_dir, seed, &format!("{name}.out"), &o.stdout);
        write_artifact(out_dir, seed, &format!("{name}.err"), &o.stderr);
    }
}

fn fuzz_seed(backends: &Backends, args: &FuzzArgs, mode: GenMode, seed: u64) -> SeedResult {
    let program = gen::generate_program(mode, seed, args.depth);
    let path = args.out.join(format!("seed-{seed}.clj"));
    if let Err(e) = std::fs::write(&path, &program) {
        return SeedResult {
            passed: false,
            detail: format!("cannot write {}: {e}", path.display()),
        };
    }
    let timeout = Duration::from_secs(args.timeout);
    let empty: Vec<String> = Vec::new();

    let jvm = backends.run_jvm_script(&path, &empty, timeout);
    let cljr = backends.run_cljr_script(&path, &empty, timeout);

    let mut problems: Vec<String> = Vec::new();
    for (name, o) in [("jvm", &jvm), ("cljr-run", &cljr)] {
        if o.timed_out {
            problems.push(format!("{name} timeout after {}s", timeout.as_secs()));
        } else if o.code != Some(0) {
            problems.push(format!(
                "{name} exit {} (want 0){}",
                o.code.map(|c| c.to_string()).unwrap_or_else(|| "?".into()),
                short_stderr(o)
            ));
        }
    }
    if problems.is_empty() && jvm.stdout != cljr.stdout {
        problems.push(format!(
            "stdout differs (jvm {} bytes, cljr-run {} bytes)",
            jvm.stdout.len(),
            cljr.stdout.len()
        ));
    }

    let mut stages: Vec<(&str, &Outcome)> = vec![("jvm", &jvm), ("cljr-run", &cljr)];
    let mut aot_run: Option<Outcome> = None;
    if mode == GenMode::Aot {
        let (build, run) = backends.run_cljr_aot(&path, timeout);
        if build.timed_out {
            problems.push(format!("aot-build timeout after {}s", timeout.as_secs()));
        } else if build.code != Some(0) {
            problems.push(format!(
                "aot-build exit {}{}",
                build
                    .code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "?".into()),
                short_stderr(&build)
            ));
        }
        stages.push(("aot-build", &build));
        if let Some(run) = &run {
            if run.timed_out {
                problems.push(format!("aot-run timeout after {}s", timeout.as_secs()));
            } else if run.code != Some(0) {
                problems.push(format!(
                    "aot-run exit {}{}",
                    run.code
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "?".into()),
                    short_stderr(run)
                ));
            } else if run.stdout != jvm.stdout {
                problems.push(format!(
                    "aot-run stdout differs ({} bytes, jvm {} bytes)",
                    run.stdout.len(),
                    jvm.stdout.len()
                ));
            }
            // Extend the borrow: stash a clone for artifact saving below.
            aot_run = Some(run.clone());
        }
        // Save build artifacts now (borrows end here).
        save_outcomes(&args.out, seed, &stages);
        if let Some(run) = &aot_run {
            save_outcomes(&args.out, seed, &[("aot-run", run)]);
        }
    } else {
        save_outcomes(&args.out, seed, &stages);
    }

    if problems.is_empty() {
        if !args.keep_pass {
            let _ = std::fs::remove_file(&path);
            for suffix in [
                "jvm.out",
                "jvm.err",
                "cljr-run.out",
                "cljr-run.err",
                "aot-build.out",
                "aot-build.err",
                "aot-run.out",
                "aot-run.err",
            ] {
                let _ = std::fs::remove_file(args.out.join(format!("seed-{seed}.{suffix}")));
            }
        }
        SeedResult {
            passed: true,
            detail: String::new(),
        }
    } else {
        SeedResult {
            passed: false,
            detail: problems.join("; "),
        }
    }
}

fn short_stderr(o: &Outcome) -> String {
    let first = String::from_utf8_lossy(&o.stderr)
        .lines()
        .next()
        .unwrap_or("")
        .to_string();
    if first.is_empty() {
        String::new()
    } else {
        let mut s: String = first.chars().take(160).collect();
        if s.len() >= 160 {
            s.push_str("...");
        }
        format!(": {s}")
    }
}

pub fn fuzz(args: FuzzArgs) -> i32 {
    let mode = match args.mode.as_str() {
        "interp" => GenMode::Interp,
        "aot" => GenMode::Aot,
        other => {
            eprintln!("bad --mode '{other}' (want interp or aot)");
            return 2;
        }
    };
    if let Err(e) = std::fs::create_dir_all(&args.out) {
        eprintln!("cannot create {}: {e}", args.out.display());
        return 2;
    }
    let jvm_classpath = crate::runner::find_jars(&args.jars);
    let cljr_bin =
        crate::runner::which(&args.clojure_rs_bin).map(|p| p.to_string_lossy().into_owned());
    if jvm_classpath.is_none() {
        eprintln!(
            "fuzz needs the JVM oracle: jars missing in {} (run scripts/fetch-clojure-jars.sh)",
            args.jars.display()
        );
        return 2;
    }
    if cljr_bin.is_none() {
        eprintln!(
            "fuzz needs the device under test: clojure-rs binary '{}' not found",
            args.clojure_rs_bin
        );
        return 2;
    }
    let backends = Arc::new(Backends::new(
        jvm_classpath,
        cljr_bin,
        args.runtime_archive.clone(),
        false, // failure artifacts are saved under --out; tmpdirs are garbage
    ));
    let xfail = match parse_seed_list(&args.xfail_seeds) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let xfail = Arc::new(xfail);
    let args = Arc::new(args);
    let print_lock = Arc::new(Mutex::new(()));
    let seeds: Vec<u64> = (args.start_seed..args.start_seed + args.count).collect();
    let n_threads = args.jobs.max(1).min(seeds.len().max(1));
    let failed: Arc<Mutex<Vec<SeedResult>>> = Arc::new(Mutex::new(Vec::new()));
    let n_xfail: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let n_xpass: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for tid in 0..n_threads {
        let backends = Arc::clone(&backends);
        let args = Arc::clone(&args);
        let xfail = Arc::clone(&xfail);
        let print_lock = Arc::clone(&print_lock);
        let failed = Arc::clone(&failed);
        let n_xfail = Arc::clone(&n_xfail);
        let n_xpass = Arc::clone(&n_xpass);
        let seeds = seeds.clone();
        handles.push(std::thread::spawn(move || {
            let mut i = tid;
            while i < seeds.len() {
                let seed = seeds[i];
                let r = fuzz_seed(&backends, &args, mode, seed);
                let expected = xfail.contains(&seed);
                {
                    let _guard = print_lock.lock().unwrap();
                    match classify_seed(r.passed, expected) {
                        Verdict::Pass => println!("PASS: seed {seed}"),
                        Verdict::XPass => {
                            println!("XPASS: seed {seed} (expected failure; unpin it)");
                            *n_xpass.lock().unwrap() += 1;
                        }
                        Verdict::XFail => {
                            println!("XFAIL: seed {seed}: {}", r.detail);
                            println!(
                                "  repro: clj-torture generate --mode {} --seed {seed} --out /tmp/repro",
                                args.mode
                            );
                            println!(
                                "  program: {}",
                                args.out.join(format!("seed-{seed}.clj")).display()
                            );
                            *n_xfail.lock().unwrap() += 1;
                        }
                        Verdict::Fail => {
                            println!("FAIL: seed {seed}: {}", r.detail);
                            println!(
                                "  repro: clj-torture generate --mode {} --seed {seed} --out /tmp/repro",
                                args.mode
                            );
                            println!(
                                "  program: {}",
                                args.out.join(format!("seed-{seed}.clj")).display()
                            );
                            failed.lock().unwrap().push(r);
                        }
                    }
                }
                i += n_threads;
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    let failed = failed.lock().unwrap();
    let n_xfail = *n_xfail.lock().unwrap();
    let n_xpass = *n_xpass.lock().unwrap();
    println!("# fuzzed seeds\t\t{}", seeds.len());
    println!("# mismatches\t\t{}", failed.len());
    println!("# expected failures\t{n_xfail}");
    println!("# unexpected passes\t{n_xpass}");
    fuzz_exit_code(failed.len(), n_xpass, args.strict_xpass)
}

#[cfg(test)]
mod tests {
    use super::parse_seed_list;

    #[test]
    fn seed_list_empty_is_empty() {
        assert!(parse_seed_list("").unwrap().is_empty());
        assert!(parse_seed_list("   ").unwrap().is_empty());
    }

    #[test]
    fn seed_list_parses_commas_and_spaces() {
        let s = parse_seed_list("2,10, 11 ,2").unwrap();
        assert_eq!(s.len(), 3);
        assert!(s.contains(&2) && s.contains(&10) && s.contains(&11));
    }

    #[test]
    fn seed_list_rejects_garbage() {
        assert!(parse_seed_list("2,x").is_err());
        assert!(parse_seed_list("1-3").is_err());
        assert!(parse_seed_list("-1").is_err());
    }

    #[test]
    fn classify_covers_all_four_verdicts() {
        use super::{classify_seed, Verdict};
        assert_eq!(classify_seed(true, false), Verdict::Pass);
        assert_eq!(classify_seed(true, true), Verdict::XPass);
        assert_eq!(classify_seed(false, true), Verdict::XFail);
        assert_eq!(classify_seed(false, false), Verdict::Fail);
    }

    #[test]
    fn exit_code_rules() {
        use super::fuzz_exit_code;
        assert_eq!(fuzz_exit_code(0, 0, false), 0);
        assert_eq!(fuzz_exit_code(0, 0, true), 0);
        assert_eq!(fuzz_exit_code(0, 2, false), 0);
        assert_eq!(fuzz_exit_code(0, 2, true), 1);
        assert_eq!(fuzz_exit_code(1, 0, false), 1);
        assert_eq!(fuzz_exit_code(1, 3, true), 1);
    }
}
