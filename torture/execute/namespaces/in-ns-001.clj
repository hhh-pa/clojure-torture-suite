;; create-ns/in-ns/intern/remove-ns/find-ns lifecycle.
;; Port: (clojure.core/refer 'clojure.core) leaves core vars (nil?)
;; unresolvable in the new ns.
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(create-ns 'torture.scratch)
(in-ns 'torture.scratch)
(clojure.core/refer 'clojure.core)
(intern 'torture.scratch 'val 123)
(assert (= val 123))
(assert (= (ns-name *ns*) 'torture.scratch))
(remove-ns 'torture.scratch)
(assert (nil? (find-ns 'torture.scratch)))
(println "TORTURE-PASS")
