;; Threading macros: -> ->> as-> cond-> cond->> some-> some->>.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (-> 5 (+ 3) (* 2)) 16))
(assert (= (->> 5 (+ 3) (* 2)) 16))
(assert (= (as-> 5 x (+ x 1) (* x x)) 36))
(assert (= (cond-> 0 true inc false (* 100) true (+ 10)) 11))
(assert (= (cond->> 0 true (+ 5) false (* 100)) 5))
(assert (= (some-> {:a 1} :a inc) 2))
(assert (nil? (some-> nil :a inc)))
(assert (= (some->> 5 (+ 3)) 8))
(println "TORTURE-PASS")
