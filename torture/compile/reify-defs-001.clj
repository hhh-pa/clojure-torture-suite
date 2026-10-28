;; A defn returning a reify instance loads cleanly.
;; { dg-do load }

(defprotocol PPrintable (render [x]))
(defn make-renderer [tag]
  (reify PPrintable (render [_] [:rendered tag])))
