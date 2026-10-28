;; Multi-arity and variadic definitions load cleanly.
;; { dg-do load }

(defn multi ([a] a) ([a b] [a b]) ([a b & cs] [a b cs]))
(defn variadic [head & tail] [head tail])
