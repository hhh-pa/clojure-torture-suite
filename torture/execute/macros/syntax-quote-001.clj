;; Syntax-quote unquote/splicing; auto-gensym freshness.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [a 1] (assert (= `(~a ~(+ a 1)) '(1 2))))
(assert (= `(1 ~@[2 3]) '(1 2 3)))
(assert (symbol? (first `(foo#))))
(assert (not= `(foo#) `(foo#)))
(println "TORTURE-PASS")
