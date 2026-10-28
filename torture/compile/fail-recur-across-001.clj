;; recur outside tail position must fail analysis on every backend.
;; { dg-do fail-load }

(defn bad [x] (+ 1 (recur 2)))
