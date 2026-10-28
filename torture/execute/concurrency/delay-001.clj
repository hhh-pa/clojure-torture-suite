;; delay/force computes once; realized? tracks it.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [calls (atom 0) d (delay (swap! calls inc) 42)]
  (assert (not (realized? d)))
  (assert (= (force d) 42))
  (assert (= (force d) 42))
  (assert (= @calls 1))
  (assert (realized? d)))
(println "TORTURE-PASS")
