;; case dispatch incl. constant-list tests and default.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (case 2 1 :one 2 :two :other) :two))
(assert (= (case 9 1 :one 2 :two :other) :other))
(assert (= (case :b :a 1 (:b :c) 2 3) 2))
(assert (= (let [x 1] (case x 1 :one)) :one))
(println "TORTURE-PASS")
