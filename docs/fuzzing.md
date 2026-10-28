# Fuzzing with generated programs

`clj-torture generate` is the Csmith of this suite: a seeded, deterministic
random Clojure program generator. `clj-torture fuzz` is the driver loop —
generate, run every backend, byte-compare. Together they torture the
[`clojure-rs`](https://github.com/hhh-pa/clojure) compiler the way Csmith
tortures GCC: endless well-formed programs, zero false positives.

## Running it

```sh
# Print one interp program (seed 42) to stdout.
./harness/target/release/clj-torture generate --mode interp --seed 42

# Emit 500 AOT programs to a directory.
./harness/target/release/clj-torture generate --mode aot --seed 1 --count 500 --out /tmp/aot-progs

# Fuzz seeds 1..200: JVM vs clojure-rs must agree on every program.
./harness/target/release/clj-torture fuzz --mode interp --start-seed 1 --count 200 \
  --clojure-rs-bin /path/to/clojure-rs

# Fuzz AOT: JVM vs clojure-rs run vs build+execute must agree three ways.
./harness/target/release/clj-torture fuzz --mode aot --start-seed 1 --count 100 \
  --clojure-rs-bin /path/to/clojure-rs
```

Generated files carry `dg-do`/`dg-mode` headers, so a failing seed's program
can be dropped straight into `torture/` as a regression test (add `dg-output`
from the JVM run, or `dg-xfail-backend` if it pins a known divergence).

## The safety contract

A fuzzer that cries wolf is worse than none. Every generated program must
exit 0 with identical stdout on all backends, enforced by construction in
`harness/src/gen.rs`:

- **type-directed**: `=` only joins identical scalar types; `/` on ints is
  never emitted (no ratios); no cross-type numerics anywhere;
- **total arithmetic**: `quot`/`rem`/`mod` divisors are nonzero literals,
  `*` operands are small, and every int expression carries a `[lo, hi]`
  range — anything escaping ±1e6 falls back to a literal (no overflow,
  no division by zero);
- **in-range indexing**: `nth`/`get`/`assoc`/`subvec` indices are literals
  proven in range (2-arity `get` only on vectors — it returns nil on
  seqs); unknown lengths only feed total operations (`count`, `conj`);
- **`subs` is ASCII-only** (indices are UTF-16 units);
- **maps and sets are never printed or sequenced** (order-free), only
  queried (`get` with default, `contains?`, `count`);
- **no floats, no ratios, no state, no I/O** besides the final `prn`
  (interp) or `print` (AOT);
- **termination**: `loop` bounds are literals; self-recursive helpers are
  only called with small literals at one dedicated call site, never from
  generated call positions;
- **AOT mode stays in-subset**: top-level fixed-arity `defn`,
  `if`/`do`/`let`, direct self-recursion, the 13 primitives
  `+ - * quot < <= > >= = count nth get conj`, and `print` — no `loop`,
  `rem`, `str`, seqs, maps, or `and`/`or` (checked by unit test
  `aot_subset_shape`).

Determinism: the only entropy is a xorshift64* stream; same seed always
emits byte-identical code (unit test `deterministic`).

## When fuzz finds a mismatch

`fuzz` exits 1 and keeps, per failing seed under `--out`:

- `seed-N.clj` — the program;
- `seed-N.{jvm,cljr-run,aot-run}.out/.err` — every backend's output;
- `seed-N.aot-build.{out,err}` — the AOT compile log (aot mode).

Plus a repro line:

```text
FAIL: seed 42: stdout differs (jvm 19 bytes, cljr-run 26 bytes)
  repro: clj-torture generate --mode interp --seed 42 --out /tmp/repro
  program: fuzz-out/seed-42.clj
```

Triage order: (1) shrink the program by hand to a minimal case; (2) decide
port bug vs generator bug (re-read the contract above — if the program
breaks a rule, the generator is wrong); (3) file against
[hhh-pa/clojure](https://github.com/hhh-pa/clojure) with the seed, or fix
`gen.rs` and add a regression shape test.

## Growing the subset

The generator deliberately covers less than the language. To add a form:

1. extend the AST/typed emitter in `gen.rs` with a total-by-construction
   rule (or a tracked invariant like lengths/ranges);
2. extend `aot_subset_shape` if the form is AOT-legal, or gate it to
   interp mode;
3. validate: `generate --count 200` for the mode, then `run` the output
   directory on the JVM backend — all 200 must pass before committing.

Good next candidates: `doseq`/`for` comprehensions, threading macros,
`try`/`catch` over generated throws, nested vectors, `update-in` paths.
