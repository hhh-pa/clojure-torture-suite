;; Nested map/vector/set/list data reads as one form.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

{:a [1 2] :b #{3} :c (quote (4))}
