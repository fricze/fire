(ns fricze.fire
  (:require
	[coffi.mem :as mem]
	[coffi.ffi :as ffi :refer [defcfn]])
  (:gen-class))

(defn greet
  "Callable entry point to the application."
  [data]
  (println (str "Hello, " (or (:name data) "World") "!")))

(defcfn strlen
  "Given a string, measures its length in bytes."
  strlen [::mem/c-string] ::mem/long)

(ffi/load-library "../target/release/libfire.dylib")

(defcfn just-string just_string [] ::mem/c-string)


(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println (just-string))
  (greet 
	{:name (strlen "dupsok")})
  (greet {:name (first args)}))

