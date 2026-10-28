;; *print-level* truncates nesting with # on both backends.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (binding [*print-level* 1] (pr-str [[1] [2]])) "[# #]"))
(println "TORTURE-PASS")
