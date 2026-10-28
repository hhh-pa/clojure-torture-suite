;; comment blocks and discards never evaluate at load.
;; { dg-do load }

(comment
  (def never 1)
  (/ 1 0)
  (throw (ex-info "nope" {})))
#_(def also-never 2)
(defn still-here [] :yes)
