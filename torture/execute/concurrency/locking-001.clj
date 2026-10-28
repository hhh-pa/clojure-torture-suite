;; locking serializes; monitors are reentrant.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [a (atom [])]
  (locking :mon (swap! a conj :in))
  (assert (= @a [:in]))
  (locking :mon (locking :mon (swap! a conj :nest)))
  (assert (= @a [:in :nest])))
(println "TORTURE-PASS")
