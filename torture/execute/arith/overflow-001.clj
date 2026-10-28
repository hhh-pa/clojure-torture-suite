;; Boundary values just inside the checked-i64 range (both impls agree).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (+ 9223372036854775807 0) 9223372036854775807))
(assert (= (- -9223372036854775808 0) -9223372036854775808))
(assert (< (* 3037000499 3037000499) 9223372036854775807))
(println "TORTURE-PASS")
