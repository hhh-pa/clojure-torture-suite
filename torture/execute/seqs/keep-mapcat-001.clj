;; keep/keep-indexed/mapcat/map-indexed.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (keep #(when (even? %) %) [1 2 3 4]) [2 4]))
(assert (= (keep even? [1 2 3 4]) [false true false true]))
(assert (= (keep-indexed (fn [i x] (when (even? i) x)) [:a :b :c]) [:a :c]))
(assert (= (mapcat reverse [[1 2] [3]]) [2 1 3]))
(assert (= (map-indexed vector [10 20]) [[0 10] [1 20]]))
(println "TORTURE-PASS")
