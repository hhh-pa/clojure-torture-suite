;; format specifiers: %d %s %x %.2f %05d.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (format "%d-%s-%x" 42 "a" 255) "42-a-ff"))
(assert (= (format "%.2f" 3.14159) "3.14"))
(assert (= (format "%05d" 42) "00042"))
(println "TORTURE-PASS")
