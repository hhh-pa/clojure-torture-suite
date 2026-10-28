;; with-in-str feeds *in* for read-line/read.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (with-in-str "hello" (read-line)) "hello"))
(assert (= (with-in-str "(1 2)" (read)) '(1 2)))
(println "TORTURE-PASS")
