;; apply spreads 5000 args; the sum is exact.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (apply + (range 5000)) 12497500))
(println "TORTURE-PASS")
