(module $main
  (type (func))
  (import "wasi" "log" (func (type 0)))
  (func $print (type 0) call 1)
  (start 0)
)