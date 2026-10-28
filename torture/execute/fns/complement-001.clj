;; complement/constantly/identity produce fns.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= ((complement even?) 3) true))
(assert (= (filter (complement nil?) [1 nil 2]) [1 2]))
(assert (= ((constantly 7) 1 2 3) 7))
(assert (= (identity 9) 9))
(println "TORTURE-PASS")
