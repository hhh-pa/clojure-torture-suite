;; Interface + deftype definitions load cleanly.
;; Port: deftype methods cannot see fields as locals
;; ("Unable to resolve symbol: nm").
;; { dg-do load }
;; { dg-xfail-backend clojure-rs }

(definterface Named (getName []))
(deftype NamedThing [nm]
  Named
  (getName [this] nm))
