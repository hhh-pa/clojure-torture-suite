;; Reader conditionals with platform branches and :default.
;; Port: read-string rejects #? ("Conditional read not allowed") — no
;; :read-cond opt-in like the JVM wrapper's. Coupled to the read
;; wrapper: if the port gains an opts map, pass it there and drop this.
;; { dg-do read }
;; { dg-xfail-backend clojure-rs }
;; { dg-output "READ-OK\n" }

#?(:clj :on-jvm :cljr :on-port :default :dflt)
