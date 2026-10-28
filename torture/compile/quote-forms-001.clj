;; Quoted and syntax-quoted data definitions load cleanly.
;; { dg-do load }

(def quoted-data '(a b (c d) [1 2] {:k :v}))
(def syntax-data `(a ~'b ~@[1 2] c#))
(def nested-quote ''(deeply (quoted)))
