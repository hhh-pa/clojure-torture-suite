;; apply spreads the final seq; nil counts as empty.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (apply + [1 2 3]) 6))
(assert (= (apply + 1 2 [3 4]) 10))
(assert (= (apply vector 1 2 nil) [1 2]))
(assert (= (apply str ["a" "b"]) "ab"))
(println "TORTURE-PASS")
