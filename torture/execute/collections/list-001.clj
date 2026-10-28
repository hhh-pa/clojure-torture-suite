;; List basics: conj/peek/pop/cons/list*.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (conj '(2 3) 1) '(1 2 3)))
(assert (= (peek '(1 2 3)) 1))
(assert (= (pop '(1 2 3)) '(2 3)))
(assert (= (cons 0 [1 2]) '(0 1 2)))
(assert (= (list* 1 2 [3 4]) '(1 2 3 4)))
(println "TORTURE-PASS")
