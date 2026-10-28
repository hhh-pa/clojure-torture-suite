;; Char literals: simple, named, and unicode.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

[\a \Z \0 \newline \space \tab \return \backspace \formfeed \u03BB]
