;; for comprehensions with :when/:let/:while and nesting.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (for [x [1 2 3]] (* x 2)) [2 4 6]))
(assert (= (for [x [1 2 3 4] :when (even? x)] x) [2 4]))
(assert (= (for [x [1 2] :let [y (* x 10)]] y) [10 20]))
(assert (= (for [x [1 2 3] :while (< x 3)] x) [1 2]))
(assert (= (for [x [1 2] y [:a :b]] [x y]) [[1 :a] [1 :b] [2 :a] [2 :b]]))
(println "TORTURE-PASS")
