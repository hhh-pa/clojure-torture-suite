;; Integer subtraction incl. unary negation.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (- 10 4 1) 5))
(assert (= (- 5) -5))
(assert (= (- 0 5) -5))
(assert (= (- -9223372036854775808 -1) -9223372036854775807))
(println "TORTURE-PASS")
