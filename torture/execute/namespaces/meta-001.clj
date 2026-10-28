;; Metadata: alter/reset on vars, with-meta/vary-meta on values.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(def with-meta-var "v")
(alter-meta! #'with-meta-var assoc :k 1)
(assert (= (:k (meta #'with-meta-var)) 1))
(reset-meta! #'with-meta-var {:k 2})
(assert (= (meta #'with-meta-var) {:k 2}))
(assert (= (meta (with-meta [1] {:a 1})) {:a 1}))
(assert (= (meta (vary-meta (with-meta [1] {:n 1}) update :n inc)) {:n 2}))
(def ^:private priv 1)
(assert (:private (meta #'priv)))
(println "TORTURE-PASS")
