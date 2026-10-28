;; ns-interns/ns-publics/ns-map; {:private true} hides from publics.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(ns torture.map-sample)
(defn pub-fn [] 1)
(defn priv-fn {:private true} [] 2)
(assert (contains? (ns-interns *ns*) 'pub-fn))
(assert (contains? (ns-publics *ns*) 'pub-fn))
(assert (not (contains? (ns-publics *ns*) 'priv-fn)))
(assert (var? ((ns-map *ns*) 'pub-fn)))
(println "TORTURE-PASS")
