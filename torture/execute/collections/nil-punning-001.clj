;; nil punning across seq/collection fns.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (conj nil 1) '(1)))
(assert (= (assoc nil :a 1) {:a 1}))
(assert (= (first nil) nil))
(assert (= (rest nil) '()))
(assert (= (count nil) 0))
(assert (= (seq nil) nil))
(assert (nil? (next nil)))
(assert (= (get nil :k :d) :d))
(println "TORTURE-PASS")
