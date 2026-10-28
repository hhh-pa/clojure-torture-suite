;; Vars: alter-var-root, var-get, dynamic binding push/pop.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(def my-var 1)
(assert (= my-var 1))
(alter-var-root #'my-var inc)
(assert (= my-var 2))
(assert (var? #'my-var))
(assert (= (var-get #'my-var) 2))
(def ^:dynamic *my-dyn* 10)
(binding [*my-dyn* 20] (assert (= *my-dyn* 20)))
(assert (= *my-dyn* 10))
(println "TORTURE-PASS")
