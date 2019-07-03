```
int factorial(int n) {
  if (n == 0)
    return 1;
  else
    return n * factorial(n-1);
}
```

```
get_local 0
i64.const 0
i64.eq
if i64
    i64.const 1
else
    get_local 0
    get_local 0
    i64.const 1
    i64.sub
    call 0
    i64.mul
end
```

```
20 00
42 00
51
04 7e
42 01
05
20 00
20 00
42 01
7d
10 00
7e
0b
```

```
(module
  (memory 256 256)
  (export "memory" memory)
  (type $FUNCSIG$dd (func (param f64) (result f64)))
  (import $exp "global.Math" "exp" (param f64) (result f64))
  (export "doubleExp" $doubleExp)
  (func $doubleExp (param $0 f64) (result f64)
    (f64.mul
      (call_import $exp
        (get_local $0)
      )
      (f64.const 2)
    )
  )
)
```
