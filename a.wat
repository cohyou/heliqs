(module $main
  (type (func (param i32) (param i32) (result i64)))
  (import "wasi" "log" (func (type 3456)))
  (func $print (type 423) (local i32) call 3456)
  (func $aaaa (type 87))
  (func $bbbb (type 221))
  (start 3456)
)