;; Stacked ^metadata prefixes read onto one form.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

^:private ^:dynamic {:tag :x} [1 2]
