;; AOT: direct self-recursion (fact/fib), vector print.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[120 55]" }

(defn fact [n] (if (= n 0) 1 (* n (fact (+ n -1)))))
(defn fib [n] (if (< n 2) n (+ (fib (+ n -1)) (fib (+ n -2)))))
(print [(fact 5) (fib 10)])
