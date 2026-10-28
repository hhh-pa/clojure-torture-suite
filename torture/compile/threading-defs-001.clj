;; Threading-macro bodies load cleanly.
;; { dg-do load }

(defn t1 [x] (-> x (+ 1) (* 2) (vector :done)))
(defn t2 [x] (->> x (map inc) (filter even?) (into [])))
(defn t3 [x] (cond-> x true (assoc :a 1) false (assoc :b 2)))
