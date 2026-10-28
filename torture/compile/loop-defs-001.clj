;; loop/recur/doseq/for bodies load cleanly unexecuted.
;; { dg-do load }

(defn sum-to [n]
  (loop [i 0 acc 0]
    (if (< i n) (recur (inc i) (+ acc i)) acc)))
(defn each-print [xs]
  (doseq [x xs] (print x)))
(defn squares [xs]
  (for [x xs] (* x x)))
