(module $moddd
  ;; types
  (type $tp1 (func))
  (type $tp2 (func (param i32) (param i64)))
  (type $tp3 (func (param f32) (param f64) (result i32)))
  (type (func (result i64)))
  (type $uniop (func (param i64)))

  ;; imports
  (import "imp_func" "1" (func $f1 (type 0)))
  (import "imp_func" "2" (func (type $tp2)))
  (import "imp_func" "3" (func (type 1) (param i32) (param i64)))
  (import "imp_func" "4" (func $fjiao (type $tp3) (param f32) (param f64) (result i32)))
  (import "imp_func" "5" (func (type 2)))
  (import "imp_func" "6" (func (type 3) (result i64)))
  ;; (import "imp_table" "b" (table $first 2 funcref))
  ;; (import "imp_table" "b" (table 54 83 funcref))
  ;; (import "imp_mem" "1" (memory $fqew 1234))
  ;; (import "imp_mem" "2" (memory 64 56849))
  ;; (import "imp_global" "1" (global $ggg f64))
  ;; (import "imp_global" "2" (global (mut i32)))

  ;; tables
  ;; (table $second 43 funcref)

  ;; mems
  ;; (memory $aaaaaa 98)

  ;; globals
  ;; (global $wowowwow i32)

  ;; funcs
  (func $fjao (type 0))
  (func $fjao (type 1))
  (func $11 (type $tp2))
  (func $fa (type 1) (param $p2 i32) (param i64))
  (func $afm (type $tp3) (param $arg1 f32) (param f64) (result i32))
  (func $aaa (type 2) (local i32))
  (func $aaa (type 2) (local $local1 i32) (local i64))
  (func $bbb (type 3) (result i64))
  (func (type 3) (result i64))
  (func (type $uniop) (param i64) (local i32))
  (func (type $tp2) (param i32) (param i64) (local i32))
  (func (type 3) (result i64) (local $ooo f64) (local $jjj i32))


  ;; exports
  ;; (export "t" (table $second))
  ;; (export "m" (memory $aaaaaa))
  ;; (export "g" (global $wowowwow))

  ;; start
  ;; (start 52)

  ;; elem
  ;; (elem 0 (offset) 1 2 3 $afm 5)

  ;; data
  ;; (data 4 (offset) "jlac84myrtqp9")
)