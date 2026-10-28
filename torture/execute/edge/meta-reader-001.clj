;; ^metadata reader form attaches to the next form.
;; Port: (meta ^:private [1]) is not {:private true}.
;; { dg-do run }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (meta ^:private [1]) {:private true}))
(assert (= (meta ^:k {:m 1}) {:k true}))
(assert (= (meta (with-meta 'sym {:a 1})) {:a 1}))
(println "TORTURE-PASS")
