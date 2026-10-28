;; trampoline bounces without growing the stack.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [f (fn f [n] (if (zero? n) :done #(f (dec n))))]
  (assert (= (trampoline f 10000) :done)))
(println "TORTURE-PASS")
