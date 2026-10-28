;; interleave/concat/interpose incl. nil inputs.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (interleave [1 2] [:a :b]) [1 :a 2 :b]))
(assert (= (take 4 (interleave [1 2] (repeat :x))) [1 :x 2 :x]))
(assert (= (concat [1] nil [2 3]) [1 2 3]))
(assert (= (interpose \, [1 2 3]) [1 \, 2 \, 3]))
(println "TORTURE-PASS")
