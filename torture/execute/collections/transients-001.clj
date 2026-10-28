;; Transient edit and persistent! across vector/map/set.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (persistent! (conj! (conj! (transient []) 1) 2)) [1 2]))
(assert (= (persistent! (assoc! (transient {}) :a 1)) {:a 1}))
(assert (= (persistent! (disj! (transient #{1 2}) 1)) #{2}))
(assert (= (persistent! (pop! (transient [1 2]))) [1]))
(println "TORTURE-PASS")
