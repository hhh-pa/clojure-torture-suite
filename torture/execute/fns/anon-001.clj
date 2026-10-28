;; #() sugar: %, %1/%2, and rest arg %&.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (#(+ %1 %2) 3 4) 7))
(assert (= (map #(inc %) [1 2]) [2 3]))
(assert (= (#(apply + %&) 1 2 3) 6))
(assert (= (#(identity %&) :a :b) '(:a :b)))
(println "TORTURE-PASS")
