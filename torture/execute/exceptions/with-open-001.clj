;; with-open closes its Closeable and returns the body value.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(let [closed (atom false)
      c (reify java.io.Closeable (close [_] (reset! closed true)))]
  (assert (= (with-open [_ c] :body) :body))
  (assert @closed))
(println "TORTURE-PASS")
