;; #'var-quote, var, and @deref compose.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defn vq-target [] :v)
(assert (= (@#'vq-target) :v))
(assert (var? #'vq-target))
(let [v #'vq-target] (assert (= (@v) :v)))
(println "TORTURE-PASS")
