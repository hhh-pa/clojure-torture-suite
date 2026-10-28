;; memoize caches per argument list.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [calls (atom 0)
      f (memoize (fn [x] (swap! calls inc) (* x 2)))]
  (assert (= (f 21) 42))
  (assert (= (f 21) 42))
  (assert (= @calls 1)))
(println "TORTURE-PASS")
