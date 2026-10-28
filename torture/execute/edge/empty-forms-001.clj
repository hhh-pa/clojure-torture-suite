;; Empty collections read and compare sanely.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= '() ()))
(assert (= [] []))
(assert (= {} {}))
(assert (nil? nil))
(assert (= (list) '()))
(println "TORTURE-PASS")
