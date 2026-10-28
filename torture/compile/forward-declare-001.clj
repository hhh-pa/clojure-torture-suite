;; declare enables mutual references; clean load is the pass.
;; { dg-do load }

(declare even?-fwd)
(defn odd?-fwd [n] (if (zero? n) false (even?-fwd (dec n))))
(defn even?-fwd [n] (if (zero? n) true (odd?-fwd (dec n))))
