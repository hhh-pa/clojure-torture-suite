;; Integer multiplication incl. zero-arity identity.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (*) 1))
(assert (= (* 7) 7))
(assert (= (* 2 3 4) 24))
(assert (= (* -3 7) -21))
(assert (= (* 1000000 1000000) 1000000000000))
(println "TORTURE-PASS")
