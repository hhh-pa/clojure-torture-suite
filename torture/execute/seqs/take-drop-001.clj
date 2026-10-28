;; take/drop families incl. -while, -last, -nth, split-at/with.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (take 2 [1 2 3]) [1 2]))
(assert (= (drop 2 [1 2 3]) [3]))
(assert (= (take 9 [1 2]) [1 2]))
(assert (= (drop 9 [1 2]) []))
(assert (= (take-while #(< % 3) [1 2 3 1]) [1 2]))
(assert (= (drop-while #(< % 3) [1 2 3 1]) [3 1]))
(assert (= (take-last 2 [1 2 3]) [2 3]))
(assert (= (drop-last 2 [1 2 3]) [1]))
(assert (= (take-nth 2 [0 1 2 3 4]) [0 2 4]))
(assert (= (split-at 2 [1 2 3 4]) [[1 2] [3 4]]))
(assert (= (split-with #(< % 3) [1 2 3 1]) [[1 2] [3 1]]))
(println "TORTURE-PASS")
