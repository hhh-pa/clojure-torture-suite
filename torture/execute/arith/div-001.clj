;; Integer division yields exact ratios on both backends.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (/ 1 2) 1/2))
(assert (ratio? (/ 1 2)))
(assert (= (/ 7 2) 7/2))
(assert (= (/ 4 2) 2))
(assert (= (/ 1 2 2) 1/4))
(println "TORTURE-PASS")
