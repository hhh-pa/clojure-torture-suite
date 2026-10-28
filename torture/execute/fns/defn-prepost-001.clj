;; defn :pre/:post conditions fire on violation.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(defn checked [x] {:pre [(pos? x)] :post [(> % x)]} (* x 2))
(assert (= (checked 2) 4))
(assert (try (checked -1) false (catch Throwable t true)))
(defn badpost [x] {:post [(< % 0)]} x)
(assert (try (badpost 5) false (catch Throwable t true)))
(println "TORTURE-PASS")
