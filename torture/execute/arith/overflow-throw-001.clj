;; Checked overflow raises on both (JVM ArithmeticException, port arithmetic error).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (try (+ 9223372036854775807 1) false (catch Throwable t true)))
(assert (try (* 3037000500 3037000500) false (catch Throwable t true)))
(assert (try (- -9223372036854775808 1) false (catch Throwable t true)))
(println "TORTURE-PASS")
