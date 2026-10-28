;; min/max/abs and numeric predicates over ints.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (min 3 1 2) 1))
(assert (= (max 3 1 2) 3))
(assert (= (min 5) 5))
(assert (= (abs -5) 5))
(assert (= (abs 5) 5))
(assert (zero? 0))
(assert (not (zero? 1)))
(assert (pos? 3))
(assert (neg? -3))
(assert (even? 4))
(assert (odd? 5))
(println "TORTURE-PASS")
