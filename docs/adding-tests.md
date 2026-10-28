# Adding tests

## Scaffold

```sh
./scripts/new-test.sh torture/execute/seqs my-test
# writes torture/execute/seqs/my-test-001.clj from the template
```

## Rules

1. **One behavior per file.** A failing test must name its culprit.
   Name: kebab-case + numeric suffix per directory (`take-003.clj`).
   Regression tests for port issues: `gh-<N>-<slug>.clj` with the issue
   URL in a comment.
2. **Self-checking.** `execute` tests `assert` every behavior and end
   with `(println "TORTURE-PASS")` pinned by `dg-output`. `compile`
   tests just need clean-loadable tops (prefer `defn` over bare
   side effects so `load` stays pure).
3. **Deterministic.** No network, no wall-clock assertions, no unseeded
   randomness. Concurrency tests: bound every wait (`deref` with
   timeout, `await-for`), keep sleeps under a second, declare
   `dg-timeout` headroom.
4. **Oracle-first.** The test must pass on JVM Clojure. If the port
   diverges, assert JVM behavior + `dg-xfail-backend clojure-rs` with a
   comment citing the divergence — see
   [torture modes](torture-modes.md). Host-only tests (Java interop,
   `clojure.java.io` file dance) take `dg-skip-backend clojure-rs`
   instead; keep those rare.
5. **Reader suite is single-form.** `torture/reader/` files hold exactly
   one readable form plus comments — the harness feeds the whole file to
   `read-string`.
6. **AOT suite stays in-subset.** `torture/aot/` tests use only the
   documented AOT subset (see [directives](directives.md#dg-mode)) and
   `dg-mode interp+aot`. No `-main` (JVM script mode would not invoke
   it — outputs would differ).
7. **Limits stay green on the JVM.** Pick depths/sizes the oracle
   survives; document the chosen boundary in a comment.

## Check before pushing

```sh
make harness   # once: builds harness/target/release/clj-torture
./harness/target/release/clj-torture run --backend jvm torture/execute/seqs/my-test-001.clj
make check     # full oracle gate, must be green
```

## Template (`execute`)

```clojure
;; One-line description of the tortured behavior.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (take 3 (range 10)) [0 1 2]))  ; hmm: take returns a seq, not a vector
(println "TORTURE-PASS")
```

(Careful: `(take 3 (range 10))` is `(0 1 2)` — but `=` is
collection-kind-agnostic for sequential collections, so the assert holds.
The suite is full of traps like this; run the oracle before committing.)
