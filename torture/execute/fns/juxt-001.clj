;; juxt applies every fn to the same args.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= ((juxt :a :b) {:a 1 :b 2}) [1 2]))
(assert (= ((juxt inc dec) 5) [6 4]))
(println "TORTURE-PASS")
