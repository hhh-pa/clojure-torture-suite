;; spit/slurp round-trip through a temp file (JVM interop: host-only).
;; { dg-do run }
;; { dg-skip-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(let [f (java.io.File/createTempFile "torture" ".txt")]
  (spit (.getPath f) "hello")
  (assert (= (slurp (.getPath f)) "hello"))
  (.delete f))
(println "TORTURE-PASS")
