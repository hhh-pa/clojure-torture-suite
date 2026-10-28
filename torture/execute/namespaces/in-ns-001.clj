;; create-ns/in-ns/intern/remove-ns/find-ns lifecycle.
;; { dg-do run }
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
