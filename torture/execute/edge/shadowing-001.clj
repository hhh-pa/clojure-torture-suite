;; Locals shadow clojure.core vars, including +.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [map 1 filter 2] (assert (= (+ map filter) 3)))
(let [+ -] (assert (= (+ 10 3) 7)))
(println "TORTURE-PASS")
