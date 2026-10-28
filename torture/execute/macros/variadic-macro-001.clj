;; Multi-arity recursive macro (or reimplementation).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmacro my-or ([] nil) ([x] x) ([x & xs] `(let [v# ~x] (if v# v# (my-or ~@xs)))))
(assert (= (my-or) nil))
(assert (= (my-or false nil 3) 3))
(assert (= (my-or false :a :b) :a))
(println "TORTURE-PASS")
