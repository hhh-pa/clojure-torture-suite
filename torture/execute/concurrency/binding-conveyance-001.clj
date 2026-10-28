;; Bindings convey to futures; with-bindings/push-pop thread bindings.
;; { dg-do run }
;; { dg-output "TORTURE-PASS\n" }

(def ^:dynamic *tcv* :root)
(let [f (binding [*tcv* :bound] (future *tcv*))]
  (assert (= @f :bound)))
(with-bindings {#'*tcv* :wb} (assert (= *tcv* :wb)))
(let [_ (push-thread-bindings {#'*tcv* :push})]
  (assert (= *tcv* :push))
  (pop-thread-bindings))
(assert (= *tcv* :root))
(shutdown-agents)
(println "TORTURE-PASS")
