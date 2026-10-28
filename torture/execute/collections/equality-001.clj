;; Sequential equality is kind-agnostic; map/set equality is structural.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= [1 2] '(1 2)))
(assert (= (take 2 [1 2 3]) [1 2]))
(assert (= {:a 1} (hash-map :a 1)))
(assert (= #{1 2} (hash-set 2 1)))
(assert (not= [1 2] [1 2 3]))
(assert (= "a" "a"))
(println "TORTURE-PASS")
