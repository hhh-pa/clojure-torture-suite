;; sort/sort-by/reverse over numbers, strings, and maps.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (sort [3 1 2]) [1 2 3]))
(assert (= (sort > [1 3 2]) [3 2 1]))
(assert (= (sort-by :n [{:n 2} {:n 1}]) [{:n 1} {:n 2}]))
(assert (= (sort ["b" "a"]) ["a" "b"]))
(assert (= (reverse [1 2 3]) [3 2 1]))
(println "TORTURE-PASS")
