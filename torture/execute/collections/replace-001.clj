;; replace swaps matches, preserving the input shape.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (replace {:a 1 :b 2} [:a :b :c]) [1 2 :c]))
(assert (= (replace {1 9} [1 2 1]) [9 2 9]))
(println "TORTURE-PASS")
