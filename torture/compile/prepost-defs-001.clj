;; :pre/:post fns load cleanly (checked at call time, not load).
;; { dg-do load }

(defn guarded [x] {:pre [(number? x)] :post [(>= % 0)]} (* x x))
