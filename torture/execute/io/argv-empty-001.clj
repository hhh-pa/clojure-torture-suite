;; No script args: *command-line-args* is nil on both impls.
;; { dg-do run }
;; { dg-output "nil\nTORTURE-PASS\n" }

(prn *command-line-args*)
(println "TORTURE-PASS")
