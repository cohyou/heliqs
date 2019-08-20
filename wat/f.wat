(module $moddd
  ;; types
  (type $tp1 (func))
  (type $tp2 (func (param i32) (param i64)))

  ;; imports
  ;; (import "imp_func" "1" (func $f1 (type 0)))
  ;; (import "imp_func" "2" (func (type $tp2)))
  ;; (import "imp_table" "b" (table $first 2 funcref))
  ;; (import "imp_table" "b" (table 54 83 funcref))
  ;; (import "imp_mem" "1" (memory $fqew 1234))
  (import "imp_mem" "2" (memory 64 56849))
  (import "imp_global" "1" (global $ggg f64))
  (import "imp_global" "2" (global (mut i32)))

  ;; tables
  (table $second 43 funcref)

  ;; mems
  (memory $aaaaaa 98)

  ;; globals
  (global $wowowwow i32)

  ;; funcs
  (func $fjao)
  (func $11)
  (func $fa)
  (func $afm)
  (func $aaa)

  ;; exports
  (export "t" (table $second))
  (export "m" (memory $aaaaaa))
  (export "g" (global $wowowwow))

  ;; start
  (start 52)

  ;; elem
  (elem 0 (offset) 1 2 3 $afm 5)

  ;; data
  (data 4 (offset) "jlac84myrtqp9")
)