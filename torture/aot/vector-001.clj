;; AOT: conj/count/nth/get incl. misses over vectors.
;; NOTE: no keywords here — only Int/Double/Bool/Str/Vector values flow in AOT.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[[1 2 3] 3 20 99 nil]" }

(print [(conj [1 2] 3) (count [1 2 3]) (nth [10 20] 1) (get [1 2] 5 99) (get [1 2] 5)])
