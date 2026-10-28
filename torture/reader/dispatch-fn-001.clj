;; Anonymous-fn dispatch macro with % args.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

#(vector % %2 %&)
