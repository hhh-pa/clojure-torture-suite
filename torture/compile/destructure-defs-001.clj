;; Complex destructuring arglists load cleanly.
;; { dg-do load }

(defn f [{{x :x} :a [y & ys] :b {:keys [z] :or {z 0}} :c}] [x y ys z])
(defn g [[a [b [c]] & {:keys [d]}]] [a b c d])
