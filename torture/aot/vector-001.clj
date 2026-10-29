;; AOT: conj/count/nth/get incl. misses over vectors; keywords flow.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[[1 2 3] 3 20 99 nil :miss true false :miss :u/id]" }

(print [(conj [1 2] 3) (count [1 2 3]) (nth [10 20] 1) (get [1 2] 5 99) (get [1 2] 5) :miss (= :a :a) (= :a :b) (get [:x :y] 5 :miss) :u/id])
