;; read-string reads the first form from a string.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (read-string "(+ 1 2)") '(+ 1 2)))
(assert (= (read-string "\"s\" :k 42") "s"))
(assert (= (read-string "[1 2]") [1 2]))
(println "TORTURE-PASS")
