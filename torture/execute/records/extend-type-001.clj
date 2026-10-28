;; extend-type/extend onto named host classes (String, Long).
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defprotocol P2 (p2 [x]))
(extend-type String P2 (p2 [x] [:str x]))
(extend Long P2 {:p2 (fn [x] [:long x])})
(assert (= (p2 "s") [:str "s"]))
(assert (= (p2 5) [:long 5]))
(assert (satisfies? P2 "s"))
(println "TORTURE-PASS")
