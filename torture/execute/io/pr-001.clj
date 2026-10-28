;; pr-str/print-str/println-str and print/read round-trip.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (pr-str "s" :k 1) "\"s\" :k 1"))
(assert (= (print-str [1 2] {:a 1}) "[1 2] {:a 1}"))
(assert (= (println-str :a) ":a\n"))
(assert (= (read-string (pr-str {:a [1 {:b 2}]})) {:a [1 {:b 2}]}))
(println "TORTURE-PASS")
