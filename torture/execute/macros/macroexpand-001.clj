;; macroexpand-1/macroexpand; syntax-quote qualifies non-specials.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (macroexpand-1 '(when true 1)) '(if true (do 1))))
(assert (= (macroexpand '(when true 1)) '(if true (do 1))))
(defmacro twice [x] `(+ ~x ~x))
(assert (= (macroexpand-1 '(twice 21)) '(clojure.core/+ 21 21)))
(println "TORTURE-PASS")
