;; AOT: nested vectors print human-style (bare strings, nil).
;; { dg-do run }
;; { dg-mode interp+aot }
;; { dg-output "[1 [2 3] s true nil]" }

(print [1 [2 3] "s" true nil])
