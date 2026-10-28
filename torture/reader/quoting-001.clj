;; Quote, nested quote, syntax-quote, deref, and var-quote read cleanly.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

'(a 'b `(c ~d ~@e) @f #'g)
