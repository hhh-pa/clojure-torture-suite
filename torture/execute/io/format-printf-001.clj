;; format and printf specifiers.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (format "%s=%d" "n" 7) "n=7"))
(assert (= (with-out-str (printf "%03d" 5)) "005"))
(println "TORTURE-PASS")
