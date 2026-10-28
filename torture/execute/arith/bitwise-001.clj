;; Bitwise lattice (out of scope on the port: extended-arith slice).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (bit-and 12 10) 8))
(assert (= (bit-or 12 10) 14))
(assert (= (bit-xor 12 10) 6))
(assert (= (bit-and-not 14 12) 2))
(assert (= (bit-not 0) -1))
(assert (= (bit-shift-left 1 10) 1024))
(assert (= (bit-shift-right -8 2) -2))
(assert (= (unsigned-bit-shift-right -8 2) 4611686018427387902))
(assert (bit-test 8 3))
(assert (= (bit-set 0 3) 8))
(assert (= (bit-clear 8 3) 0))
(assert (= (bit-flip 0 3) 8))
(println "TORTURE-PASS")
