;; every?/some/not-any?/not-every?; some returns the pred value.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (every? even? [2 4]))
(assert (not (every? even? [2 3])))
(assert (= (some even? [1 3 4]) true))
(assert (= (some #{3} [1 2 3]) 3))
(assert (nil? (some even? [1 3])))
(assert (not-any? odd? [2 4]))
(assert (not-every? even? [2 3]))
(println "TORTURE-PASS")
