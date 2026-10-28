;; String escapes and non-ASCII content read cleanly.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

["a\nb\tc\"d\\e" "λ" "𝄞"]
