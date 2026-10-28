;; Integer addition: arities and multi-arg sums.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (+) 0))
(assert (= (+ 1) 1))
(assert (= (+ 1 2 3 4) 10))
(assert (= (+ -5 5) 0))
(assert (= (+ 9223372036854775806 1) 9223372036854775807))
(println "TORTURE-PASS")
