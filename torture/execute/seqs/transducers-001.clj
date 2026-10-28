;; Transducers: comp of xforms, transduce, sequence, eduction, halt-when.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (into [] (comp (map inc) (filter even?)) [1 2 3 4]) [2 4]))
(assert (= (transduce (map inc) + 0 [1 2 3]) 9))
(assert (= (sequence (map inc) [1 2]) [2 3]))
(assert (= (seq (eduction (map inc) [1 2])) [2 3]))
(assert (= (transduce (halt-when odd?) conj [] [2 4 5 6]) 5))
(println "TORTURE-PASS")
