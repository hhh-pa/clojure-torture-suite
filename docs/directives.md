# Directive reference

Directives live in `;;` comments, one `{ dg-... }` per comment, anywhere
before the first non-comment form (the harness scans only leading comment
lines). Unknown `dg-` directives are a harness error — fail loudly, like
DejaGNU.

## `dg-do`

What the harness demands. Default: `run`.

- `run` — test must exit 0. If `dg-output` / `dg-output-re` is present,
  stdout must also match.
- `load` — file must load without error (exit 0). No output check.
  (The `compile/` suite convention; legal anywhere.)
- `read` — the file body must read cleanly (single form; comments
  skipped). Implemented as `(read-string (slurp <file>))` + `READ-OK`
  sentinel, so it works identically on both backends.
- `fail-run` — test must exit nonzero. Optional `dg-error-re` pins stderr.
- `fail-load` — file must fail to load (exit nonzero). Optional
  `dg-error-re` pins stderr.
- `fail-read` — the file body must fail to read (exit nonzero). Optional
  `dg-error-re` pins stderr.

Example:

```clojure
;; { dg-do fail-read }
;; { dg-error-re "Unmatched delimiter" }
)
```

## `dg-mode`

Which `clojure-rs` execution paths apply. Default: `interp`.

- `interp` — `clojure-rs run` only.
- `aot` — `clojure-rs build` + native run only.
- `interp+aot` — both, each must pass independently (plus identical stdout
  when `--diff-backends` is on).

The `jvm` backend ignores `dg-mode` (it runs every test once in script
mode). Tests in `torture/aot/` must use `interp+aot` and stay inside the
documented AOT subset (top-level fixed-arity `defn`, `if`/`do`/`let`,
direct self-recursion, the 13 primitives
`+ - * quot < <= > >= = count nth get conj`, and `print`).

## `dg-output` / `dg-output-re`

- `dg-output "..."` — stdout must equal the string exactly. C escapes
  (`\n` `\t` `\\` `\"`) are interpreted; keep the trailing `\n` that
  `println` emits.
- `dg-output-re "..."` — stdout must match the Rust regex (search, not
  full-match). Prefer `dg-output` unless the text contains paths, timings,
  or platform spellings.

At most one of the two per test.

## `dg-error-re`

Rust regex that stderr must match (search). Only meaningful with
`fail-*`; ignored otherwise. Keep patterns loose — diagnostic wording
differs between backends (e.g. JVM stack traces vs `file:line:col:`
chains).

## `dg-timeout`

Per-test timeout in seconds (default 30, harness `--timeout` overrides
nothing — the directive wins when present). Concurrency tests that sleep
should declare headroom.

## `dg-xfail-backend` / `dg-skip-backend`

- `dg-xfail-backend <backend...>` — the test is *expected* to fail on the
  named backends (`jvm`, `clojure-rs`). An expected failure reports
  `XFAIL` (not a failure); an unexpected pass reports `XPASS`, which fails
  the run only under `--strict-xpass`. Use for documented port
  divergences, with a comment citing the divergence.
- `dg-skip-backend <backend...>` — do not run on the named backends at
  all (`UNSUPPORTED`). Use for host-only tests (e.g. JVM interop that has
  no meaning on the native port).

```clojure
;; Ratio literals: the port has no ratios (documented divergence).
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "1/2\n" }
(println (/ 1 2))
```

## `dg-args`

Whitespace-separated words passed as script arguments, bound to
`*command-line-args*` (JVM: `clojure.main <file> <args...>`;
`clojure-rs run <file> <args...>`). Quote with double quotes for words
containing spaces.

```clojure
;; { dg-do run }
;; { dg-args a "b c" }
;; { dg-output "(\"a\" \"b c\")\n" }
(prn *command-line-args*)
```
