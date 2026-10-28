;; and/or return values, not booleans; empty arities.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (and 1 2 3) 3))
(assert (= (and 1 nil 3) nil))
(assert (= (and) true))
(assert (= (or nil false 3) 3))
(assert (= (or) nil))
(assert (= (or false nil) nil))
(println "TORTURE-PASS")
