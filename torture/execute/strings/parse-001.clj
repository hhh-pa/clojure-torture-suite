;; parse-long/parse-double/parse-boolean; nil on garbage.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (parse-long "42") 42))
(assert (= (parse-double "1.5") 1.5))
(assert (= (parse-boolean "true") true))
(assert (nil? (parse-long "xx")))
(assert (nil? (parse-double "xx")))
(println "TORTURE-PASS")
