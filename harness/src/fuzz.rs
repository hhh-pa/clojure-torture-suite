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
}

struct SeedResult {
    passed: bool,
    detail: String,
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
        true, // keep AOT tmpdirs on mismatch paths is handled by artifacts; keep simple
    ));
    let args = Arc::new(args);
    let print_lock = Arc::new(Mutex::new(()));
    let seeds: Vec<u64> = (args.start_seed..args.start_seed + args.count).collect();
    let n_threads = args.jobs.max(1).min(seeds.len().max(1));
    let failed: Arc<Mutex<Vec<SeedResult>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for tid in 0..n_threads {
        let backends = Arc::clone(&backends);
        let args = Arc::clone(&args);
        let print_lock = Arc::clone(&print_lock);
        let failed = Arc::clone(&failed);
        let seeds = seeds.clone();
        handles.push(std::thread::spawn(move || {
            let mut i = tid;
            while i < seeds.len() {
                let seed = seeds[i];
                let r = fuzz_seed(&backends, &args, mode, seed);
                {
                    let _guard = print_lock.lock().unwrap();
                    if r.passed {
                        println!("PASS: seed {seed}");
                    } else {
                        println!("FAIL: seed {seed}: {}", r.detail);
                        println!(
                            "  repro: clj-torture generate --mode {} --seed {seed} --out /tmp/repro",
                            args.mode
                        );
                        println!("  program: {}", args.out.join(format!("seed-{seed}.clj")).display());
                    }
                }
                if !r.passed {
                    failed.lock().unwrap().push(r);
                }
                i += n_threads;
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    let failed = failed.lock().unwrap();
    println!("# fuzzed seeds\t\t{}", seeds.len());
    println!("# mismatches\t\t{}", failed.len());
    if failed.is_empty() {
        0
    } else {
        1
    }
}
