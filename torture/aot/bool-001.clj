;; AOT: boolean flow through comparisons.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[false true]" }

(print [(= 1 2) (< 1 2)])
