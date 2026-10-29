;; eval, load-string, and read-string round-trip.
;; Port: load-string over multiple forms does not return the last value.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (eval '(+ 1 2)) 3))
(assert (= (load-string "(def load-str-val 41) (inc load-str-val)") 42))
(assert (= load-str-val 41))
(assert (= (read-string "(+ 1 2)") '(+ 1 2)))
(println "TORTURE-PASS")
