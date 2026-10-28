;; 100-deep vector literal reads; get-in walks the spine.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (get-in [[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[:leaf]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] (repeat 100 0)) :leaf))
(println "TORTURE-PASS")
