;; Top-level syntax-quote with unquote, splicing, and auto-gensym.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

`(a ~b ~@(list c) d#)
