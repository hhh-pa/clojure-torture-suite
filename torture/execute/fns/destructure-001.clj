;; Sequential/map destructuring: & :as :keys :or nesting.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [[a b & r] [1 2 3 4]] (assert (= [a b r] [1 2 [3 4]])))
(let [{:keys [x y] :or {y 9}} {:x 1}] (assert (= [x y] [1 9])))
(let [{a :a [b c] :pair {d :d} :nest} {:a 1 :pair [2 3] :nest {:d 4}}]
  (assert (= [a b c d] [1 2 3 4])))
(let [[a :as all] [1 2]] (assert (= [a all] [1 [1 2]])))
(defn f [{[x] :v [y & ys] :w}] [x y ys])
(assert (= (f {:v [10] :w [20 30]}) [10 20 (quote (30))]))
(println "TORTURE-PASS")
