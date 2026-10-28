;; Record/protocol/extension definitions load cleanly.
;; { dg-do load }

(defprotocol Sized (size-of [x]))
(defrecord Box [w h]
  Sized
  (size-of [b] (* (:w b) (:h b))))
(extend-protocol Sized nil (size-of [_] 0))
