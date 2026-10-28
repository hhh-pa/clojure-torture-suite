# GCC → Clojure torture mapping

How each GCC testsuite concept translates here. GCC references are to
`gcc/testsuite/` in a GCC checkout.

## Suites

| GCC | Here | Notes |
|---|---|---|
| `gcc.c-torture/compile/*.c` | `torture/compile/*.clj` | must load/evaluate without error; no output check |
| `gcc.c-torture/execute/*.c` | `torture/execute/**/*.clj` | must run to exit 0; `dg-output` pins stdout |
| `gcc.dg/cpp-*`, `gcc.dg/pch-*` (front-end-only) | `torture/reader/*.clj` | must read (`dg-do read`) or must fail (`dg-do fail-read`); single form per file |
| `gcc.target/i386/*` etc. (target-specific) | `torture/aot/*.clj` | valid LLVM-AOT-subset programs; also run under the interpreter |
| `gcc.c-torture/compile/limits-*.c` | `torture/limits/*.clj` | nesting / size stress near documented limits |
| `gcc.c-torture/unsorted/*` | (none — categorize instead) | every test lives in exactly one suite |

## Torture options

GCC compiles each torture test at `-O0 -O1 -O2 -O3 -Os -O0 -g -O2 -g`.
There is no `-O` knob on a Clojure evaluator, so the torture axis here is
**backend × mode**:

| GCC axis | Here |
|---|---|
| `-O0` (unoptimized baseline) | `jvm` backend: upstream Clojure, the oracle |
| `-O2` (the thing being tortured) | `clojure-rs` interp (`run`) |
| `-O3` / `-Os` (alternate codegen) | `clojure-rs` AOT (`build` + native run) for `dg-mode aot` / `interp+aot` |
| `dg-do compile` vs `dg-do run` | `dg-do load` vs `dg-do run` (plus `read`, `fail-*`) |

`--diff-backends` is the analogue of comparing `-O0` vs `-O2` execution
output: both backends must print byte-identical stdout.

## Directives

DejaGNU directives keep their names where the meaning survives:

| GCC (`dg-`) | Here | Notes |
|---|---|---|
| `dg-do compile` | `dg-do load` | file must load without error |
| `dg-do run` | `dg-do run` | must exit 0 (+ optional output check) |
| `dg-output` | `dg-output` | exact stdout match, C escapes |
| (none) | `dg-output-re` | stdout regex match |
| `dg-error` | `dg-error-re` | stderr regex match for `fail-*` tests |
| `dg-timeout` | `dg-timeout` | per-test seconds |
| `dg-xfail-if` | `dg-xfail-backend` | expected failure on named backends |
| `dg-skip-if` / `dg-require-*` | `dg-skip-backend` | do not run on named backends |
| (target selector) | `dg-mode` | `interp` / `aot` / `interp+aot` |
| (argv) | `dg-args` | words bound to `*command-line-args*` |

`dg-do preprocess` has no analogue (there is no preprocessor); `read` /
`fail-read` cover the reader the way `dg-do preprocess` covers `cpp`.

## Failure style

GCC execute tests call `abort()` on mismatch and return 0 on success.
Clojure torture tests call `(assert ...)` (throws on mismatch, harness sees
exit 1) and end with a `(println "TORTURE-PASS")` sentinel pinned by
`dg-output`. `dg-do load` tests have no sentinel — clean load is the pass.

## Regression naming

GCC names regression tests after PRs (`pr12345.c`). Here, regression tests
for port issues are named after the upstream issue, e.g.
`torture/execute/concurrency/gh-42-agent-await.clj`, with the issue URL in a
comment. Pure torture tests use descriptive kebab-case names with a numeric
suffix per directory (`add-001.clj`).
