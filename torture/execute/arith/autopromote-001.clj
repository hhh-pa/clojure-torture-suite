;; Auto-promoting +'/-'/*' (out of scope on the port: no BigInt promotion).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (+' 9223372036854775807 1) 9223372036854775808N))
(assert (= (-' -9223372036854775808 1) -9223372036854775809N))
(assert (= (*' 3037000500 3037000500) 9223372037000250000N))
(println "TORTURE-PASS")
