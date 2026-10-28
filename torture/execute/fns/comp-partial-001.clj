;; comp chains right-to-left; partial fixes leading args.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= ((comp inc inc) 1) 3))
(assert (= ((comp str +) 1 2) "3"))
(assert (= ((partial + 10) 1 2) 13))
(assert (= ((partial map inc) [1 2]) [2 3]))
(println "TORTURE-PASS")
