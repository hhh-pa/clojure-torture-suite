;; print/println/prn spacing and newline behavior.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (with-out-str (print "a" 1)) "a 1"))
(assert (= (with-out-str (println "a" 1)) "a 1\n"))
(assert (= (with-out-str (prn [1 :a])) "[1 :a]\n"))
(assert (= (with-out-str (print nil)) "nil"))
(assert (= (with-out-str (newline)) "\n"))
(println "TORTURE-PASS")
