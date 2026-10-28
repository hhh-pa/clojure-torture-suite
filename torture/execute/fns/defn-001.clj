;; defn arities: fixed, multi-arity, variadic.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defn f0 [] :zero)
(defn f1 [x] x)
(defn fmulti ([x] [:one x]) ([x y] [:two x y]) ([x y & zs] [:many x y zs]))
(assert (= (f0) :zero))
(assert (= (f1 42) 42))
(assert (= (fmulti 1) [:one 1]))
(assert (= (fmulti 1 2) [:two 1 2]))
(assert (= (fmulti 1 2 3 4) [:many 1 2 (quote (3 4))]))
(println "TORTURE-PASS")
