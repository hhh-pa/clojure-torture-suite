;; resolve/ns-resolve map symbols to vars; misses give nil.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(ns torture.resolve-sample)
(defn target [x] [:hit x])
(ns torture.resolve-user)
(alias 'rs 'torture.resolve-sample)
(assert (= ((resolve 'rs/target) 1) [:hit 1]))
(assert (= ((ns-resolve 'torture.resolve-sample 'target) 2) [:hit 2]))
(assert (nil? (resolve 'no.such/var)))
(println "TORTURE-PASS")
