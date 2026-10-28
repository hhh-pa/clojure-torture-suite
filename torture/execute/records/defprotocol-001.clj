;; defprotocol with inline record impls and a nil extension.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defprotocol Shape (area [s]) (sides [s]))
(defrecord Circle [r]
  Shape
  (area [s] (* 3 (:r s) (:r s)))
  (sides [_] 0))
(defrecord Square [w]
  Shape
  (area [s] (* (:w s) (:w s)))
  (sides [_] 4))
(extend-protocol Shape nil (area [_] -1) (sides [_] -1))
(assert (= (area (->Circle 2)) 12))
(assert (= (sides (->Square 3)) 4))
(assert (= (area nil) -1))
(assert (satisfies? Shape (->Circle 1)))
(assert (satisfies? Shape nil))
(println "TORTURE-PASS")
