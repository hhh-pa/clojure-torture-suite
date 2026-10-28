;; Regex literals and re-find/re-matches/re-seq incl. groups.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(assert (= (re-find #"\d+" "abc123def") "123"))
(assert (= (re-matches #"[a-z]+" "abc") "abc"))
(assert (nil? (re-matches #"[a-z]+" "abc123")))
(assert (= (re-seq #"\d" "a1b2") ["1" "2"]))
(assert (= (rest (re-find #"(\w+)@(\w+)" "a@b")) ["a" "b"]))
(println "TORTURE-PASS")
