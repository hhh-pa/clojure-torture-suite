;; 100-deep vector literal reads; get-in walks the spine.
;; Port: get-in down the 100-deep spine (seq path) misses.
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (get-in [[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[:leaf]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] (repeat 100 0)) :leaf))
(println "TORTURE-PASS")
