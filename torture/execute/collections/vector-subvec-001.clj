;; subvec windows with one and two bounds.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (subvec [0 1 2 3 4] 1 4) [1 2 3]))
(assert (= (subvec [0 1 2 3 4] 2) [2 3 4]))
(assert (= (subvec [0 1 2] 0 0) []))
(println "TORTURE-PASS")
