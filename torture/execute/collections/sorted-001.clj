;; Sorted collections: order, rseq, subseq, rsubseq.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (keys (sorted-map :b 2 :a 1)) '(:a :b)))
(assert (= (seq (sorted-set 3 1 2)) '(1 2 3)))
(assert (= (rseq (sorted-set 1 2 3)) '(3 2 1)))
(assert (= (subseq (sorted-map :a 1 :b 2 :c 3) >= :b) '([:b 2] [:c 3])))
(assert (= (rsubseq (sorted-set 1 2 3 4) < 3) '(2 1)))
(println "TORTURE-PASS")
