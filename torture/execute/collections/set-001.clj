;; Set basics: conj/disj/contains?/get/set.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (conj #{1 2} 3 2) #{1 2 3}))
(assert (= (disj #{1 2 3} 2) #{1 3}))
(assert (= (contains? #{:a} :a) true))
(assert (= (get #{:a :b} :b) :b))
(assert (= (set [1 2 2 3]) #{1 2 3}))
(println "TORTURE-PASS")
