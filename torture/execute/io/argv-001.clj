;; Script args land in *command-line-args* as a seq of strings.
;; { dg-do run }
;; { dg-args a "b c" }
;; { dg-output "(\"a\" \"b c\")\nTORTURE-PASS\n" }

(prn *command-line-args*)
(println "TORTURE-PASS")
