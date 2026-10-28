;; AOT: chained comparisons and same-category = over ints.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[true true true false true]" }

(print [(< 1 2) (<= 2 2) (> 3 2) (>= 3 4) (= 5 5)])
