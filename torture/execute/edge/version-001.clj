;; clojure-version reports a string (value differs per impl; never pinned).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (string? (clojure-version)))
(println "TORTURE-PASS")
