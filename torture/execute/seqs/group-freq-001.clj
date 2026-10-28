;; group-by and frequencies over seqs and strings.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (group-by even? [1 2 3 4]) {false [1 3] true [2 4]}))
(assert (= (frequencies "aab") {\a 2 \b 1}))
(println "TORTURE-PASS")
