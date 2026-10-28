;; AOT: linear defn call chain (no mutual recursion in-subset).
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "31" }

(defn h [x] (+ x 1))
(defn g [x] (h (* x 2)))
(defn f [x] (g (+ x 10)))
(print (f 5))
