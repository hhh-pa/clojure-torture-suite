;; inc/dec incl. mapped over a vector.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (inc 41) 42))
(assert (= (dec 42) 41))
(assert (= (inc -1) 0))
(assert (= (map inc [1 2 3]) [2 3 4]))
(println "TORTURE-PASS")
