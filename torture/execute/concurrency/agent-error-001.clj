;; :continue error mode survives a failing action.
;; { dg-do run }
;; { dg-timeout 60 }
;; { dg-output "TORTURE-PASS\n" }

(let [a (agent 0 :error-mode :continue)]
  (send a (fn [_] (throw (ex-info "x" {}))))
  (assert (= (await-for 5000 a) true))
  (assert (= @a 0))
  (send a inc)
  (assert (= (await-for 5000 a) true))
  (assert (= @a 1))
  (shutdown-agents))
(println "TORTURE-PASS")
