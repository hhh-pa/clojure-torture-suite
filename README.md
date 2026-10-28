# clojure-torture-suite

A [GCC torture suite](https://gcc.gnu.org/onlinedocs/gccint/C-Tests.html)
equivalent for Clojure: hundreds of small, self-contained `.clj` programs that
stress the reader, evaluator, and ahead-of-time compiler the way
`gcc/testsuite/gcc.c-torture/` stresses GCC — plus a Csmith-style random
program generator and differential fuzzer.

Device under test: [hhh-pa/clojure](https://github.com/hhh-pa/clojure)
(`clojure-rs`, the native Rust port). Oracle: upstream JVM Clojure — every
`torture/execute` test must pass there, and every generated program must print
byte-identical output on both.

## Layout

Like GCC, tests are grouped by what the harness demands of them:

| Directory | GCC analogue | Harness demand |
|---|---|---|
| `torture/execute/` | `gcc.c-torture/execute/` | run to exit 0, optional stdout check |
| `torture/compile/` | `gcc.c-torture/compile/` | load without error (no output check) |
| `torture/reader/` | `gcc.dg/cpp-*` (preprocess) | file must read (`dg-do read`) or must fail (`fail-read`) |
| `torture/aot/` | `gcc.target/*` | also survive `clojure-rs build` + native run |
| `torture/limits/` | `compile/limits-*` | deep / large inputs near implementation limits |
| `harness/` | `gcc/testsuite/lib/` | the `clj-torture` Rust tool: `run`, `generate`, `fuzz` |

Each test is one file with DejaGNU-style directives in `;;` comments:

```clojure
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "TORTURE-PASS\n" }

(defn fact [n] (if (= n 0) 1 (* n (fact (dec n)))))
(assert (= (fact 5) 120))
(println "TORTURE-PASS")
```

Full reference: [docs/directives.md](docs/directives.md).
GCC concept mapping: [docs/gcc-mapping.md](docs/gcc-mapping.md).

## Quick start

Prerequisites: Rust stable, Java 8+ (JVM oracle backend).

```sh
make check          # build harness, fetch oracle jars, run corpus on the JVM
make test           # harness unit tests (directives, generator shape)
```

Against a `clojure-rs` build (interp + AOT + differential vs the JVM):

```sh
make check-all CLOJURE_RS_BIN=/path/to/clojure-rs
```

Fuzz the port with 200 generated programs per mode:

```sh
make fuzz CLOJURE_RS_BIN=/path/to/clojure-rs        # interp differential
make fuzz-aot CLOJURE_RS_BIN=/path/to/clojure-rs    # JVM vs run vs AOT binary
```

See [docs/fuzzing.md](docs/fuzzing.md) for the generator's safety contract,
repro workflow, and how to grow its subset.

## The harness

`clj-torture run` discovers `torture/**/*.clj`, parses directives, and runs
each test on each selected backend:

- `jvm` — `java -cp harness/lib/* clojure.main <test>` (script mode, the oracle).
- `clojure-rs` — `clojure-rs run <test>` for every test, plus
  `clojure-rs build <test> -o <tmpbin>` + native execution for
  `dg-mode aot` / `interp+aot` tests.

Results print as DejaGNU-style `PASS` / `FAIL` / `XFAIL` / `XPASS` /
`UNSUPPORTED` lines with a trailing `results.sum` summary. Exit status is 0
unless a test fails (see `--strict-xpass` for expected-fail promotion).

`clj-torture generate --mode interp|aot --seed N` prints one seeded,
deterministic torture program; `clj-torture fuzz` loops that over a seed
range and diffs every backend on each program.

## Contributing

New tests welcome — one behavior per file, self-checking via `assert`, no
network, no randomness without a fixed seed. Scaffold + conventions:

```sh
./scripts/new-test.sh torture/execute/seqs my-test
```

Read [docs/adding-tests.md](docs/adding-tests.md) before writing.

## CI

`.github/workflows/torture.yml` runs two jobs: the JVM oracle suite (corpus +
harness unit tests), and the full `clojure-rs` suite (builds
[hhh-pa/clojure](https://github.com/hhh-pa/clojure) at `master` with its own
LLVM setup action, then runs interp + AOT + differential + both fuzz modes).
The `clojure-rs` job is the port's external torture gate.

## License

Eclipse Public License 1.0 (`epl-v10.html`), matching upstream Clojure.
