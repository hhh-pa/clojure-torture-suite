;; declare/forward refs, intern, ns-unmap.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(declare forward-ref)
(defn uses-forward [] (forward-ref))
(defn forward-ref [] :defined)
(assert (= (uses-forward) :defined))
(intern *ns* 'dyn 7)
(assert (= dyn 7))
(ns-unmap *ns* 'dyn)
(assert (nil? (ns-resolve *ns* 'dyn)))
(println "TORTURE-PASS")
