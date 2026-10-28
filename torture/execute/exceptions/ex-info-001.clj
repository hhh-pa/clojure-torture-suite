;; ex-info/ex-data/ex-message/ex-cause round-trip.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [e (ex-info "msg" {:a 1} (ex-info "root" {}))]
  (assert (= (ex-message e) "msg"))
  (assert (= (ex-data e) {:a 1}))
  (assert (= (ex-message (ex-cause e)) "root")))
(println "TORTURE-PASS")
