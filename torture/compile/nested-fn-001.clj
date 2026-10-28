;; Nested closures capturing multiple scopes load cleanly.
;; { dg-do load }

(defn outer [a]
  (let [b (* a 2)]
    (fn [c]
      (let [d (+ b c)]
        (fn [e] (+ a b c d e))))))
