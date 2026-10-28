;; char/int coercions and char predicates.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (char 65) \A))
(assert (= (int \A) 65))
(assert (= (str \h \i) "hi"))
(assert (char? \x))
(assert (not (char? "x")))
(println "TORTURE-PASS")
