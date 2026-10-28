;; AOT: double arithmetic and same-category = over doubles.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[3.75 6.0 true]" }

(print [(+ 1.5 2.25) (* 2.0 3.0) (= 1.5 1.5)])
