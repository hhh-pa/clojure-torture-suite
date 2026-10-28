;; subs slicing with one and two indices.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (subs "hello" 1 4) "ell"))
(assert (= (subs "hello" 2) "llo"))
(assert (= (subs "hello" 0 0) ""))
(println "TORTURE-PASS")
