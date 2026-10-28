;; dotimes binds 0..n-1 in order.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [a (atom [])]
  (dotimes [i 3] (swap! a conj i))
  (assert (= @a [0 1 2])))
(println "TORTURE-PASS")
