;; Record equality, map view, hash consistency (values never pinned).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defrecord Pair [a b])
(assert (= (->Pair 1 2) (->Pair 1 2)))
(assert (not= (->Pair 1 2) (->Pair 1 3)))
(assert (= (into {} (->Pair 1 2)) {:a 1 :b 2}))
(assert (= (hash (->Pair 1 2)) (hash (->Pair 1 2))))
(assert (map? (->Pair 1 2)))
(println "TORTURE-PASS")
