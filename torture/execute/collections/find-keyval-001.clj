;; key/val of find; keys/vals compared as sets (map order is free).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (set (keys {:b 2 :a 1})) #{:a :b}))
(assert (= (set (vals {:b 2 :a 1})) #{1 2}))
(assert (= (key (find {:a 1} :a)) :a))
(assert (= (val (find {:a 1} :a)) 1))
(println "TORTURE-PASS")
