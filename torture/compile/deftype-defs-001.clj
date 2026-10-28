;; Interface + deftype definitions load cleanly.
;; { dg-do load }

(definterface Named (getName []))
(deftype NamedThing [nm]
  Named
  (getName [this] nm))
