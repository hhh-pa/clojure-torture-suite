;; AOT: conj/count/nth/get incl. misses over vectors.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[[1 2 3] 3 20 :miss nil]" }

(print [(conj [1 2] 3) (count [1 2 3]) (nth [10 20] 1) (get [1 2] 5 :miss) (get [1 2] 5)])
