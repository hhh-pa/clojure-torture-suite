;; while loops on a mutable condition.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [a (atom 0) n (atom 0)]
  (while (< @a 3) (swap! n + @a) (swap! a inc))
  (assert (= @n 3)))
(println "TORTURE-PASS")
