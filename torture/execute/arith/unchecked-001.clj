;; unchecked-* wraps (out of scope on the port: extended-arith slice).
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (unchecked-add 9223372036854775807 1) -9223372036854775808))
(assert (= (unchecked-subtract -9223372036854775808 1) 9223372036854775807))
(assert (= (unchecked-multiply 1000000 1000000) 1000000000000))
(println "TORTURE-PASS")
