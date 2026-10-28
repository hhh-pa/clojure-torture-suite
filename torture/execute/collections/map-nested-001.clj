;; Nested map paths: assoc-in/update-in/get-in.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (assoc-in {} [:a :b :c] 1) {:a {:b {:c 1}}}))
(assert (= (update-in {:a {:n 1}} [:a :n] + 10) {:a {:n 11}}))
(assert (= (get-in {:a {:b 2}} [:a :b]) 2))
(assert (= (get-in {:a {}} [:a :zzz] :d) :d))
(println "TORTURE-PASS")
