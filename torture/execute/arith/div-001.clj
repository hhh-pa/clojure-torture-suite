;; JVM integer division yields exact ratios; the port does double division.
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (/ 1 2) 1/2))
(assert (ratio? (/ 1 2)))
(assert (= (/ 7 2) 7/2))
(assert (= (/ 4 2) 2))
(assert (= (/ 1 2 2) 1/4))
(println "TORTURE-PASS")
