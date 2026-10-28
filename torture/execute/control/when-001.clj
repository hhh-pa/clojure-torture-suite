;; when/when-not/when-let/if-let/if-some/when-some/when-first.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (when true 1 2) 2))
(assert (nil? (when false 1)))
(assert (= (when-not false :x) :x))
(assert (nil? (when-not true :x)))
(assert (= (when-let [x 5] (* x 2)) 10))
(assert (nil? (when-let [x nil] 1)))
(assert (= (if-let [x 5] x :none) 5))
(assert (= (if-let [x nil] x :none) :none))
(assert (= (if-some [x false] :was :none) :was))
(assert (nil? (when-some [x nil] 1)))
(assert (= (when-first [x [1 2]] x) 1))
(assert (nil? (when-first [x []] 1)))
(println "TORTURE-PASS")
