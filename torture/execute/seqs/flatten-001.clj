;; flatten strips nesting (sequential only; maps are entries).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (flatten [1 [2 [3]]]) [1 2 3]))
(assert (= (flatten []) []))
(assert (= (flatten {:a 1}) []))
(println "TORTURE-PASS")
