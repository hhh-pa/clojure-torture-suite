;; Hierarchy-aware dispatch needs derive/isa? (port slice S13, out of scope).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(derive ::rect ::shape)
(assert (isa? ::rect ::shape))
(defmulti area2 :kind)
(defmethod area2 ::shape [_] :shaped)
(assert (= (area2 {:kind ::rect}) :shaped))
(underive ::rect ::shape)
(assert (not (isa? ::rect ::shape)))
(println "TORTURE-PASS")
