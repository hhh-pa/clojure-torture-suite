;; cond and condp clause selection incl. :else fallthrough.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (cond (= 1 2) :no (= 1 1) :yes :else :els) :yes))
(assert (= (cond :else 42) 42))
(assert (nil? (cond false 1)))
(assert (= (condp = (+ 1 1) 1 :one 2 :two :other) :two))
(println "TORTURE-PASS")
