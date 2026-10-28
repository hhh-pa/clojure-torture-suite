;; zipmap and frequencies build maps from seqs.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (zipmap [:a :b] [1 2]) {:a 1 :b 2}))
(assert (= (frequencies [:a :b :a]) {:a 2 :b 1}))
(println "TORTURE-PASS")
