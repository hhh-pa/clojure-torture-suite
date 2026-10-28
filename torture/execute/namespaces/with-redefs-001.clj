;; with-redefs swaps roots temporarily, restores after.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defn redef-target [x] [:orig x])
(with-redefs [redef-target (fn [x] [:mock x])]
  (assert (= (redef-target 1) [:mock 1])))
(assert (= (redef-target 1) [:orig 1]))
(println "TORTURE-PASS")
