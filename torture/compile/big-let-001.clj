;; A fn with a wide let binding vector loads cleanly.
;; { dg-do load }

(defn wide [x]
  (let [a0 x a1 1 a2 2 a3 3 a4 4 a5 5 a6 6 a7 7
        a8 8 a9 9 b0 0 b1 1 b2 2 b3 3 b4 4 b5 5]
    (+ a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 b0 b1 b2 b3 b4 b5)))
