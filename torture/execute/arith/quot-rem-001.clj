;; quot/rem truncate; mod follows the divisor sign.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (quot 7 3) 2))
(assert (= (quot -7 3) -2))
(assert (= (rem -7 3) -1))
(assert (= (rem 7 -3) 1))
(assert (= (mod -7 3) 2))
(assert (= (mod 7 -3) -2))
(println "TORTURE-PASS")
