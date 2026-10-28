;; gensym returns fresh symbols, optionally prefixed.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (symbol? (gensym)))
(assert (symbol? (gensym "pre")))
(assert (not= (gensym "x") (gensym "x")))
(println "TORTURE-PASS")
