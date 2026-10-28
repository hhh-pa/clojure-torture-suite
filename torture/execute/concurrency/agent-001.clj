;; send + await-for + deref; shutdown-agents drains the pools.
;; { dg-do run }
;; { dg-timeout 60 }
;; { dg-output "TORTURE-PASS\n" }

(let [a (agent 0)]
  (send a + 1 2)
  (assert (= (await-for 5000 a) true))
  (assert (= @a 3))
  (shutdown-agents))
(println "TORTURE-PASS")
