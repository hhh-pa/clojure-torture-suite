;; Line comments inside and around the form are skipped.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

;; a full-line comment
[:a ; trailing comment
 :b]
