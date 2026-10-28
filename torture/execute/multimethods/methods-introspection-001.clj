;; methods/get-method/remove-method table introspection.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defmulti probe :k)
(defmethod probe :a [_] 1)
(defmethod probe :b [_] 2)
(assert (= (set (keys (methods probe))) #{:a :b}))
(assert (= ((get-method probe :a) {:k :a}) 1))
(remove-method probe :a)
(assert (= (set (keys (methods probe))) #{:b}))
(assert (nil? (get-method probe :zz)))
(println "TORTURE-PASS")
