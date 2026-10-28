;; String escapes and unicode literals.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= "\u03BB" "λ"))
(assert (= (count "a\nb") 3))
(assert (= (nth "a\"b" 1) \"))
(assert (= "\\" (str \\)))
(println "TORTURE-PASS")
