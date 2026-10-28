;; doseq nesting with :when and :while modifiers.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (with-out-str (doseq [x [1 2 3]] (print x))) "123"))
(assert (= (with-out-str (doseq [x [1 2 3 4] :when (even? x)] (print x))) "24"))
(assert (= (with-out-str (doseq [x [1 2] y [:a :b]] (print (str x y)))) "1:a1:b2:a2:b"))
(assert (= (with-out-str (doseq [x [1 2 3] :while (< x 3)] (print x))) "12"))
(println "TORTURE-PASS")
