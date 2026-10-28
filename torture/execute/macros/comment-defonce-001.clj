;; comment skips; defonce keeps the first root.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defonce my-once 42)
(defonce my-once 99)
(assert (= my-once 42))
(comment (def my-once :wrong))
(assert (= my-once 42))
(println "TORTURE-PASS")
