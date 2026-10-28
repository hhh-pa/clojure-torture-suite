;; Reader conditionals with platform branches and :default.
;; { dg-do read }
;; { dg-output "READ-OK\n" }

#?(:clj :on-jvm :cljr :on-port :default :dflt)
