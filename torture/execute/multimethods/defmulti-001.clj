;; defmulti dispatch on keyword fn with :default fallback.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmulti encounter :kind)
(defmethod encounter :cat [_] :meow)
(defmethod encounter :dog [_] :woof)
(defmethod encounter :default [_] :unknown)
(assert (= (encounter {:kind :cat}) :meow))
(assert (= (encounter {:kind :dog}) :woof))
(assert (= (encounter {:kind :fish}) :unknown))
(println "TORTURE-PASS")
