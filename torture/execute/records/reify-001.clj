;; reify protocol + Object toString override.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defprotocol R (rval [x y]))
(let [r (reify R (rval [_ y] [:got y]) Object (toString [_] "R!"))]
  (assert (= (rval r 9) [:got 9]))
  (assert (= (str r) "R!"))
  (assert (satisfies? R r)))
(println "TORTURE-PASS")
