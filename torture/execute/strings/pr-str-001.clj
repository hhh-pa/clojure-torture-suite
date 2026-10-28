;; pr-str shapes and print/read round-trip.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (pr-str [1 "a" :k]) "[1 \"a\" :k]"))
(assert (= (pr-str {:a 1}) "{:a 1}"))
(assert (= (with-out-str (pr [1 2])) "[1 2]"))
(assert (= (read-string (pr-str {:a [1 2]})) {:a [1 2]}))
(println "TORTURE-PASS")
