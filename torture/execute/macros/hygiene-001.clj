;; Auto-gensym locals do not clobber user bindings.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmacro swap-macro! [a b] `(let [t# @~a] (reset! ~a @~b) (reset! ~b t#)))
(let [x (atom 1) y (atom 2) t 999]
  (swap-macro! x y)
  (assert (= [@x @y t] [2 1 999])))
(println "TORTURE-PASS")
