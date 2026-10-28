;; Map basics: assoc/dissoc/get/find/merge/update/keyword lookup.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (assoc {:a 1} :b 2) {:a 1 :b 2}))
(assert (= (dissoc {:a 1 :b 2} :a) {:b 2}))
(assert (= (get {:a 1} :a) 1))
(assert (= (get {:a 1} :zzz :dflt) :dflt))
(assert (= (:a {:a 1}) 1))
(assert (= (:zzz {:a 1} 42) 42))
(assert (= (contains? {:a 1} :a) true))
(assert (= (find {:a 1} :a) [:a 1]))
(assert (= (merge {:a 1} {:b 2} {:a 9}) {:a 9 :b 2}))
(assert (= (merge-with + {:a 1} {:a 2 :b 3}) {:a 3 :b 3}))
(assert (= (select-keys {:a 1 :b 2 :c 3} [:a :c]) {:a 1 :c 3}))
(assert (= (update {:a 1} :a inc) {:a 2}))
(println "TORTURE-PASS")
