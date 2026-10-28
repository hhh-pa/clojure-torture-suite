;; futures, promises, deref timeouts, cached future failures.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [f (future (+ 1 2))]
  (assert (= @f 3))
  (assert (realized? f)))
(let [p (promise)]
  (deliver p :val)
  (assert (= @p :val))
  (assert (realized? p)))
(let [p (promise)]
  (assert (= (deref p 50 :timeout) :timeout)))
(let [f (future (throw (ex-info "boom" {})))]
  (assert (try @f false (catch Throwable t true))))
(shutdown-agents)
(println "TORTURE-PASS")
