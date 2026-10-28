;; Multimethod table definitions load cleanly.
;; { dg-do load }

(defmulti render :type)
(defmethod render :circle [m] [:c (:r m)])
(defmethod render :default [m] [:unknown (:type m)])
