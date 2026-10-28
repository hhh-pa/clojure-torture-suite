;; throw unwinds to the nearest catch with its payload.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (try (throw (ex-info "t" {})) false (catch Throwable t true)))
(let [v (try (throw (ex-info "k" {:code 7})) (catch Throwable t (ex-data t)))]
  (assert (= v {:code 7})))
(println "TORTURE-PASS")
