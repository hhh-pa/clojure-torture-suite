;; loop/recur with multiple bindings.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (loop [i 0 acc 0] (if (< i 5) (recur (inc i) (+ acc i)) acc)) 10))
(assert (= (loop [xs [1 2 3] out []] (if (empty? xs) out (recur (rest xs) (conj out (first xs))))) [1 2 3]))
(println "TORTURE-PASS")
