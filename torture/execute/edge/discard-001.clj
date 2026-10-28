;; #_ discards the next form, even a throwing one.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= [1 #_2 3] [1 3]))
(assert (= (do #_(throw (ex-info "x" {})) :ok) :ok))
(println "TORTURE-PASS")
