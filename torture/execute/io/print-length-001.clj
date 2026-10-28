;; *print-length* truncates long collections with ....
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (binding [*print-length* 2] (pr-str [1 2 3])) "[1 2 ...]"))
(println "TORTURE-PASS")
