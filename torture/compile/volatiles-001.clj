;; Volatile definitions and updates load and run at top level.
;; { dg-do load }

(def counter (volatile! 0))
(vswap! counter + 5)
(def snapshot @counter)
