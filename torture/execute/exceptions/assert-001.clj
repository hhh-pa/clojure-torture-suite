;; assert passes silently, throws on falsity, honors messages.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= 1 1))
(assert (try (assert (= 1 2)) false (catch Throwable t true)))
(assert (try (assert (= 1 2) "msg") false (catch Throwable t true)))
(println "TORTURE-PASS")
