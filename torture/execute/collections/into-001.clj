;; into across targets incl. the transducer arity.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (into [] [1 2]) [1 2]))
(assert (= (into {} [[:a 1]]) {:a 1}))
(assert (= (into #{} [1 1 2]) #{1 2}))
(assert (= (into [] (map inc) [1 2 3]) [2 3 4]))
(assert (= (into () [1 2 3]) '(3 2 1)))
(println "TORTURE-PASS")
