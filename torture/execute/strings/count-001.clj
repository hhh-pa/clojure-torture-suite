;; count on strings; astral chars count UTF-16 units on both impls.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (count "hello") 5))
(assert (= (count "") 0))
(assert (= (count "𝄞") 2))
(println "TORTURE-PASS")
