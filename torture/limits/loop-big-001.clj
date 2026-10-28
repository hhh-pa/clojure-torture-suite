;; loop/recur runs 100000 iterations without growing the stack.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (loop [i 0 s 0] (if (< i 100000) (recur (inc i) (+ s i)) s)) 4999950000))
(println "TORTURE-PASS")
