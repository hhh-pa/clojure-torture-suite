;; doto threads the target as every form's first arg, returns it.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= @(doto (atom 1) (reset! 5) (swap! inc)) 6))
(println "TORTURE-PASS")
