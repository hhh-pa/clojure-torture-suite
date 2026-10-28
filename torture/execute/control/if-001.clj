;; if truthiness: only nil and false are falsey.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (if true 1 2) 1))
(assert (= (if nil 1 2) 2))
(assert (= (if false 1 2) 2))
(assert (= (if 0 :t :f) :t))
(assert (= (if "" :t :f) :t))
(assert (= (if [] :t :f) :t))
(assert (= (if 0.0 :t :f) :t))
(assert (nil? (if false 1)))
(println "TORTURE-PASS")
