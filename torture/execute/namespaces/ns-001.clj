;; ns switches *ns*; defs land in the new namespace.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(ns torture.ns-sample)
(defn sample-fn [x] (* x 2))
(assert (= (sample-fn 21) 42))
(assert (= (ns-name *ns*) 'torture.ns-sample))
(println "TORTURE-PASS")
