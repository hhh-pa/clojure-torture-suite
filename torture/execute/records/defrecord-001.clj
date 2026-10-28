;; defrecord factories, keyword access, assoc/dissoc map behavior.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defrecord Point [x y])
(let [p (->Point 1 2)]
  (assert (= (:x p) 1))
  (assert (= (:y p) 2))
  (assert (= p (map->Point {:x 1 :y 2})))
  (assert (= (assoc p :z 3) (map->Point {:x 1 :y 2 :z 3})))
  (assert (= (dissoc p :x) {:y 2}))
  (assert (= (count p) 2))
  (assert (record? p)))
(println "TORTURE-PASS")
