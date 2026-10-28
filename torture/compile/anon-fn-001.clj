;; #() literals in various positions load cleanly.
;; { dg-do load }

(def inc* #(inc %))
(def add #(apply + %&))
(def handlers {:a #(vector :a %) :b #(%2 %1)})
