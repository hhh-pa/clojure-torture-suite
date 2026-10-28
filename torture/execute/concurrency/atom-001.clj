;; Atoms: swap!/reset!/compare-and-set!, validators, watches.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [a (atom 0)]
  (assert (= (swap! a + 5) 5))
  (assert (= (reset! a 10) 10))
  (assert (= (compare-and-set! a 10 11) true))
  (assert (= @a 11))
  (assert (= (compare-and-set! a 10 99) false))
  (assert (= @a 11)))
(let [seen (atom []) a (atom 1 :validator pos?)]
  (add-watch a :w (fn [_ _ o n] (swap! seen conj [o n])))
  (swap! a inc)
  (assert (= @seen [[1 2]]))
  (assert (try (reset! a -5) false (catch Throwable t true)))
  (remove-watch a :w)
  (swap! a inc)
  (assert (= @seen [[1 2]])))
(println "TORTURE-PASS")
