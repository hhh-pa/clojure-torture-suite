;; Vector basics: conj/peek/pop/nth/get/assoc/count.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (conj [1 2] 3) [1 2 3]))
(assert (= (peek [1 2 3]) 3))
(assert (= (pop [1 2 3]) [1 2]))
(assert (= (nth [1 2 3] 0) 1))
(assert (= (nth [1 2 3] 9 :miss) :miss))
(assert (= (count [1 2 3]) 3))
(assert (= (get [1 2 3] 1) 2))
(assert (= (assoc [1 2 3] 1 9) [1 9 3]))
(println "TORTURE-PASS")
