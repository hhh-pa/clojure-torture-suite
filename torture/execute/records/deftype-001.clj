;; definterface + deftype with Type. construction and .method call.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(definterface Greeter (greet []))
(deftype Person [fname]
  Greeter
  (greet [this] (str "hi " fname)))
(let [p (Person. "Al")]
  (assert (= (.greet p) "hi Al"))
  (assert (= (.greet (Person. "Bo")) "hi Bo")))
(println "TORTURE-PASS")
