;; defmacro with syntax-quote expansion; macroexpand-1 shape.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmacro my-when [t & body] `(if ~t (do ~@body)))
(assert (= (my-when true 1 2) 2))
(assert (nil? (my-when false 1)))
(assert (= (macroexpand-1 '(my-when true 1)) '(if true (do 1))))
(println "TORTURE-PASS")
