;; lazy-seq realization, realized?, doall, dorun.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (take 3 (lazy-seq (cons 1 (lazy-seq [2 3])))) [1 2 3]))
(let [s (map inc [1 2 3])]
  (assert (not (realized? s)))
  (assert (= (doall s) [2 3 4]))
  (assert (realized? s)))
(assert (= (with-out-str (dorun (map print [1 2]))) "12"))
(println "TORTURE-PASS")
