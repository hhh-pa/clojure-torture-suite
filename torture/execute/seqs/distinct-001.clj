;; distinct drops all dupes; dedupe drops adjacent runs.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (distinct [1 2 1 3 2]) [1 2 3]))
(assert (= (dedupe [1 1 2 3 3 2]) [1 2 3 2]))
(println "TORTURE-PASS")
