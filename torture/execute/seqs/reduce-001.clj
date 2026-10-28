;; reduce/reductions/reduce-kv and early exit via reduced.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (reduce + 0 [1 2 3]) 6))
(assert (= (reduce + [1 2 3]) 6))
(assert (= (reduce conj [] [3 1 2]) [3 1 2]))
(assert (= (reductions + [1 2 3]) [1 3 6]))
(assert (= (reduce (fn [_ x] (if (> x 2) (reduced :stop) :go)) :go [1 2 5 6]) :stop))
(assert (= (reduce-kv (fn [m k v] (assoc m k (* v 2))) {} {:a 1 :b 2}) {:a 2 :b 4}))
(println "TORTURE-PASS")
