;; Macro definitions (never expanded here) load cleanly.
;; { dg-do load }

(defmacro defpair [name a b] `(do (def ~name [~a ~b]) '~name))
(defmacro with-temp [binding & body] `(let [~binding (atom nil)] ~@body))
