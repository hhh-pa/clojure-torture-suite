;; Definitions carrying metadata and docstrings load cleanly.
;; { dg-do load }

(defn documented "does things" {:added "1.0" :private true} [x] x)
(def ^:dynamic *dyn-thing* 1)
(def ^:private hidden 2)
(defmacro doc-macro "macro doc" [x] x)
