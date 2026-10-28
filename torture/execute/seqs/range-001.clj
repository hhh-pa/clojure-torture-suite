;; range bounds, steps, and the infinite range.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (range 4) [0 1 2 3]))
(assert (= (range 1 6 2) [1 3 5]))
(assert (= (range 3 0 -1) [3 2 1]))
(assert (= (range 0) []))
(assert (= (take 3 (range)) [0 1 2]))
(println "TORTURE-PASS")
