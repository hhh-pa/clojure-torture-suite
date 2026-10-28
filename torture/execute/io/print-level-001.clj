;; JVM *print-level* truncates nesting with #; the port uses ... (documented).
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (binding [*print-level* 1] (pr-str [[1] [2]])) "[# #]"))
(println "TORTURE-PASS")
