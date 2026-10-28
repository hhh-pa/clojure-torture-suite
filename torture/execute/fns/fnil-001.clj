;; fnil substitutes defaults for nil arguments.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= ((fnil + 100) nil 5) 105))
(assert (= ((fnil vector :dflt) nil) [:dflt]))
(println "TORTURE-PASS")
