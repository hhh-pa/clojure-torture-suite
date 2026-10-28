;; JVM = is same-category for numbers (INTEGER/FLOATING/RATIO/DECIMAL);
;; == is the cross-category comparison. The port agrees (not a divergence).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (not (= 1 1.0)))
(assert (== 1 1.0))
(assert (= 1 1))
(assert (= 1.0 1.0))
(assert (not= 1 2))
(println "TORTURE-PASS")
