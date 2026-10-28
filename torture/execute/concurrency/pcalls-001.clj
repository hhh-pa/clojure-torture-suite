;; pcalls/pvalues run in parallel; results compared as sets.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (set (pcalls #(inc 1) #(inc 2))) #{2 3}))
(assert (= (set (pvalues 1 (+ 1 1))) #{1 2}))
(shutdown-agents)
(println "TORTURE-PASS")
