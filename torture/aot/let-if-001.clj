;; AOT: let shadowing and truthiness (nil false; 0 "" true).
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[2 2 1 1]" }

(print [(let [x 1 x 2] x) (if nil 1 2) (if 0 1 2) (if "" 1 2)])
