;; alias and refer wire vars across namespaces.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(ns torture.alias-a)
(defn f [x] [:a x])
(ns torture.alias-b)
(defn f [x] [:b x])
(ns torture.alias-user)
(alias 'a 'torture.alias-a)
(refer 'torture.alias-b :only '[f])
(assert (= (a/f 1) [:a 1]))
(assert (= (f 2) [:b 2]))
(println "TORTURE-PASS")
