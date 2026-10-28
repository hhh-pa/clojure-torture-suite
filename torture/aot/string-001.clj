;; AOT: string flow and count, printed human-style.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[hi 5 0]" }

(print ["hi" (count "hello") (count "")])
