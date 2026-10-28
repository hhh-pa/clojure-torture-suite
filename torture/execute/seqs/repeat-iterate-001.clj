;; repeat/iterate/cycle/repeatedly produce lazy infinite seqs.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (take 3 (repeat 7)) [7 7 7]))
(assert (= (repeat 2 :x) [:x :x]))
(assert (= (take 4 (iterate inc 0)) [0 1 2 3]))
(assert (= (take 5 (cycle [1 2])) [1 2 1 2 1]))
(assert (= (take 3 (repeatedly (fn [] 9))) [9 9 9]))
(println "TORTURE-PASS")
