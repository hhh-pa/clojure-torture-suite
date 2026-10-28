;; AOT: quot truncation incl. negatives.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[2 -2]" }

(print [(quot 7 3) (quot -7 3)])
