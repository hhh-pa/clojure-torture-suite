;; try/catch/finally bodies load cleanly.
;; { dg-do load }

(defn safe-div [a b]
  (try (/ a b)
       (catch Throwable t [:error (ex-message t)])
       (finally nil)))
