(module $main
  (type (func))
  (type $i32ret (func (result i32)))
  ;; (func $f1add (type $i32ret) i32.const 42 i32.const 8734 i32.add)
  (func $f1sub (type $i32ret) i32.const 90672598 i32.const 2346789 i32.sub)
  (func $f1mul (type $i32ret) i32.const 3 i32.const 4 i32.mul)
  ;; (func $f1div (type $i32ret) i32.const 45 i32.const 5 i32.div_u)
  (start $f1mul)
)