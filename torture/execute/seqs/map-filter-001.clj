;; map/filter/remove incl. multi-coll map and eager -v variants.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (map inc [1 2 3]) [2 3 4]))
(assert (= (map + [1 2] [10 20] [100 200]) [111 222]))
(assert (= (mapv inc [1 2]) [2 3]))
(assert (vector? (mapv inc [1])))
(assert (= (filter even? (range 6)) [0 2 4]))
(assert (= (filterv odd? (range 5)) [1 3]))
(assert (= (remove even? (range 6)) [1 3 5]))
(println "TORTURE-PASS")
