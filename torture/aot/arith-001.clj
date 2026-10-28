;; AOT: defn calls, + - * quot, print without newline.
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "sumsq=25" }

(defn sq [x] (* x x))
(defn sumsq [a b] (+ (sq a) (sq b)))
(print "sumsq=")
(print (sumsq 3 4))
