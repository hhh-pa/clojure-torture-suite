;; defmethod with multiple arity bodies.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmulti msum (fn [op & _] op))
(defmethod msum :add ([_ a b] (+ a b)) ([_ a b c] (+ a b c)))
(assert (= (msum :add 1 2) 3))
(assert (= (msum :add 1 2 3) 6))
(println "TORTURE-PASS")
