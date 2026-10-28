;; Unicode symbols and names survive read and eval.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defn λ [x] (* x 2))
(assert (= (λ 21) 42))
(let [café 1] (assert (= café 1)))
(assert (= "λ" "\u03BB"))
(println "TORTURE-PASS")
