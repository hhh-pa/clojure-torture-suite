;; Double arithmetic, == cross-category compare, NaN/infinite predicates.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (== 1 1.0))
(assert (= 0.5 (/ 1.0 2)))
(assert (= 3.75 (+ 1.5 2.25)))
(assert (NaN? (/ 0.0 0.0)))
(assert (infinite? (/ 1.0 0.0)))
(assert (not (infinite? 1.5)))
(println "TORTURE-PASS")
