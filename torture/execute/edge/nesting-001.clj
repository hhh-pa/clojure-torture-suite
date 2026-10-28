;; Nested calls and deep data paths evaluate inside-out.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= ((comp inc inc inc) 0) 3))
(assert (= (get-in {:a {:b {:c 42}}} [:a :b :c]) 42))
(println "TORTURE-PASS")
