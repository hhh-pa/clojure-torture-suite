;; str concatenates printed values; nil prints as "".
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (str 1 "a" :k 's nil) "1a:ks"))
(assert (= (str nil) ""))
(assert (= (str) ""))
(println "TORTURE-PASS")
