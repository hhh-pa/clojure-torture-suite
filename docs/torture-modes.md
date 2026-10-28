# Torture modes and backends

## Backends

| Name | Command | Role |
|---|---|---|
| `jvm` | `java -cp harness/lib/* clojure.main` | oracle: upstream Clojure semantics |
| `clojure-rs` | `clojure-rs` on `PATH` or `--clojure-rs-bin` | device under test |

Select with `--backend jvm|clojure-rs|all` (default: `jvm`).
A backend whose binary/jars are missing skips its tests as `UNSUPPORTED`
with a one-line warning — never silently green.

### `jvm`

- `run` / `fail-run`: script mode — `clojure.main <test> <dg-args...>`.
  Top-level forms execute in order; `*command-line-args*` is bound
  (a seq, or `nil` with no args — same as the port).
- `load` / `fail-load`: `-e "(load-file \"<test>\")"`.
- `read` / `fail-read`: `-e` wrapper doing
  `(read-string (slurp "<test>"))` then printing `READ-OK`.

Clojure 1.12.x jars live in `harness/lib/` (see
`scripts/fetch-clojure-jars.sh`); override with `--jars <dir>`.

### `clojure-rs`

- `run` / `fail-run`: `clojure-rs run <test> <dg-args...>`.
- `load` / `fail-load`: same as `run` (the port loads by running tops).
- `read` / `fail-read`: a generated wrapper script run via `run`
  (`read-string` + `slurp` are both ported).
- `dg-mode aot` / `interp+aot`: additionally
  `clojure-rs build <test> -o <tmpbin>` then execute `<tmpbin>`
  (no args — `*command-line-args*` is unconveyed in AOT binaries).
  Override the runtime archive with `--runtime-archive`.

Exit-code contract (both backends): 0 pass, nonzero fail. The JVM backend
relies on `clojure.main` exiting 1 on uncaught throwables; `clojure-rs`
exits 1 on evaluation/compile errors per its spec.

## Differential mode

`--diff-backends` runs every non-skipped test on all selected backends and
additionally requires byte-identical stdout across them (and across
interp/AOT within `clojure-rs`). This is the `-O0`-vs-`-O2` output
comparison of GCC torture. Tests whose output legitimately differs per
backend (error text, object printing) should use per-test `dg-output`
only where the text is stable, or `dg-output-re` with a loose pattern —
a differential mismatch on such a test is a test bug, not a port bug.

## What "pass" means per suite

- `torture/execute/`: exit 0 on the JVM **always** (a JVM failure is a
  test bug — fix the test, not the port). On `clojure-rs`: exit 0 unless
  the test pins a documented divergence with `dg-xfail-backend`.
- `torture/compile/`: clean load on both backends (same xfail rule).
- `torture/reader/`: `read` succeeds / `fail-read` fails on both
  backends. Reader error *text* is not compared across backends.
- `torture/aot/`: interp and AOT agree byte-for-byte on stdout.
- `torture/limits/`: passes on the JVM; `clojure-rs` xfail where the
  port documents a lower limit (cite it in a comment).

## Oracle discipline

JVM Clojure is the specification. When the port deliberately diverges
(unchecked or higher-precision arithmetic, unimplemented vars like the
bitwise lattice, …), the torture test asserts **JVM behavior** and
carries `dg-xfail-backend clojure-rs`
with a comment naming the divergence. If the port later converges, the
test flips to `XPASS` and `--strict-xpass` (on in CI for the port job)
forces someone to drop the marker. Never write a test that asserts
port-only behavior without also skipping the JVM — an oracle that fails
is a broken suite.
