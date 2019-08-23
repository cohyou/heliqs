(module $main
  (type (func))
  (type (func (param i32)))
  (import "wasi" "log" (func (type 1)))
  (func $print (type 0))
  (start 1)
)