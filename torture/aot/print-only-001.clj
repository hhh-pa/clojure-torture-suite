;; AOT: multi-arg print space-joins; bare print of mixed values.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "a 1 [2 3]" }

(print "a" 1 [2 3])
