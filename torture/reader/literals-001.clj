;; Core literals read in one vector: numbers, strings, chars, bools, nil.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

[42 -7 3.14 -0.5 "str" \a \newline true false nil :kw ::foo sym / *+]
