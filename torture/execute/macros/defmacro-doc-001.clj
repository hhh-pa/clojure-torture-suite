;; defmacro docstring and attr-map land on the var metadata.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmacro with-doc "docstring here" {:added "1.0"} [x] x)
(assert (= (:doc (meta #'with-doc)) "docstring here"))
(assert (= (:added (meta #'with-doc)) "1.0"))
(println "TORTURE-PASS")
