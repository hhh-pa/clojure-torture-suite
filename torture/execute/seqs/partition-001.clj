;; partition/partition-all/partition-by incl. step and ragged tails.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (partition 2 [1 2 3 4]) [[1 2] [3 4]]))
(assert (= (partition-all 3 [1 2 3 4]) [[1 2 3] [4]]))
(assert (= (partition-by even? [1 1 2 2 3]) [[1 1] [2 2] [3]]))
(assert (= (partition 2 1 [1 2 3]) [[1 2] [2 3]]))
(println "TORTURE-PASS")
