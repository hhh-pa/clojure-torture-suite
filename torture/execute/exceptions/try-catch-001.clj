;; try/catch/finally: values, ex-data recovery, finally ordering.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (try 1 (catch Throwable t 2) (finally nil)) 1))
(assert (= (try (throw (ex-info "e" {:n 1})) (catch Throwable t (:n (ex-data t)))) 1))
(let [a (atom [])]
  (try (throw (ex-info "x" {})) (catch Throwable t (swap! a conj :caught)) (finally (swap! a conj :fin)))
  (assert (= @a [:caught :fin])))
(println "TORTURE-PASS")
