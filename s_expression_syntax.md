```
num:    <digit> (_? <digit>)*
hexnum: <hexdigit> (_? <hexdigit>)*
nat:    <num> | 0x<hexnum>
int:    <nat> | +<nat> | -<nat>
float:  <num>.<num>?(e|E <num>)? | 0x<hexnum>.<hexnum>?(p|P <num>)?
name:   $(<letter> | <digit> | _ | . | + | - | * | / | \ | ^ | ~ | = | < | > | ! | ? | @ | # | $ | % | & | | | : | ' | `)+
string: "(<char> | \n | \t | \\ | \' | \" | \<hex><hex> | \u{<hex>+})*"

value:  <int> | <float>
var:    <nat> | <name>

unop:  ctz | clz | popcnt | ...
binop: add | sub | mul | ...
relop: eq | ne | lt | ...
sign:  s|u
offset: offset=<nat>
align: align=(1|2|4|8|...)
cvtop: trunc | extend | wrap | ...

val_type: i32 | i64 | f32 | f64
elem_type: funcref
block_type : ( result <val_type>* )*
func_type:   ( type <var> )? <param>* <result>*
global_type: <val_type> | ( mut <val_type> )
table_type:  <nat> <nat>? <elem_type>
memory_type: <nat> <nat>?

expr:
  ( <op> )
  ( <op> <expr>+ )                                                   ;; = <expr>+ (<op>)
  ( block <name>? <block_type> <instr>* )
  ( loop <name>? <block_type> <instr>* )
  ( if <name>? <block_type> ( then <instr>* ) ( else <instr>* )? )
  ( if <name>? <block_type> <expr>+ ( then <instr>* ) ( else <instr>* )? ) ;; = <expr>+ (if <name>? <block_type> (then <instr>*) (else <instr>*)?)

instr:
  <expr>
  <op>                                                               ;; = (<op>)
  block <name>? <block_type> <instr>* end <name>?                    ;; = (block <name>? <block_type> <instr>*)
  loop <name>? <block_type> <instr>* end <name>?                     ;; = (loop <name>? <block_type> <instr>*)
  if <name>? <block_type> <instr>* end <name>?                       ;; = (if <name>? <block_type> (then <instr>*))
  if <name>? <block_type> <instr>* else <name>? <instr>* end <name>? ;; = (if <name>? <block_type> (then <instr>*) (else <instr>*))

op:
unreachable
nop
br <var>
br_if <var>
br_table <var>+
return
call <var>
call_indirect <func_type>
drop
select
local.get <var>
local.set <var>
local.tee <var>
global.get <var>
global.set <var>
<val_type>.load((8|16|32)_<sign>)? <offset>? <align>?
<val_type>.store(8|16|32)? <offset>? <align>?
memory.size
memory.grow
<val_type>.const <value>
<val_type>.<unop>
<val_type>.<binop>
<val_type>.<testop>
<val_type>.<relop>
<val_type>.<cvtop>_<val_type>(_<sign>)?

func:    ( func <name>? <func_type> <local>* <instr>* )
         ( func <name>? ( export <string> ) <...> )                         ;; = (export <string> (func <N>)) (func <name>? <...>)
         ( func <name>? ( import <string> <string> ) <func_type>)           ;; = (import <name>? <string> <string> (func <func_type>))
param:   ( param <val_type>* ) | ( param <name> <val_type> )
result:  ( result <val_type>* )
local:   ( local <val_type>* ) | ( local <name> <val_type> )

global:  ( global <name>? <global_type> <instr>* )
         ( global <name>? ( export <string> ) <...> )                       ;; = (export <string> (global <N>)) (global <name>? <...>)
         ( global <name>? ( import <string> <string> ) <global_type> )      ;; = (import <name>? <string> <string> (global <global_type>))
table:   ( table <name>? <table_type> )
         ( table <name>? ( export <string> ) <...> )                        ;; = (export <string> (table <N>)) (table <name>? <...>)
         ( table <name>? ( import <string> <string> ) <table_type> )        ;; = (import <name>? <string> <string> (table <table_type>))
         ( table <name>? ( export <string> )* <elem_type> ( elem <var>* ) ) ;; = (table <name>? ( export <string> )* <size> <size> <elem_type>) (elem (i32.const 0) <var>*)
elem:    ( elem <var>? (offset <instr>* ) <var>* )
         ( elem <var>? <expr> <var>* )                                      ;; = (elem <var>? (offset <expr>) <var>*)
memory:  ( memory <name>? <memory_type> )
         ( memory <name>? ( export <string> ) <...> )                       ;; = (export <string> (memory <N>))+ (memory <name>? <...>)
         ( memory <name>? ( import <string> <string> ) <memory_type> )      ;; = (import <name>? <string> <string> (memory <memory_type>))
         ( memory <name>? ( export <string> )* ( data <string>* ) )         ;; = (memory <name>? ( export <string> )* <size> <size>) (data (i32.const 0) <string>*)
data:    ( data <var>? ( offset <instr>* ) <string>* )
         ( data <var>? <expr> <string>* )                                   ;; = (data <var>? (offset <expr>) <string>*)

start:   ( start <var> )

typedef: ( type <name>? ( func <param>* <result>* ) )

import:  ( import <string> <string> <imkind> )
imkind:  ( func <name>? <func_type> )
         ( global <name>? <global_type> )
         ( table <name>? <table_type> )
         ( memory <name>? <memory_type> )

export:  ( export <string> <exkind> )
exkind:  ( func <var> )
         ( global <var> )
         ( table <var> )
         ( memory <var> )

module:  ( module <name>? <typedef>* <func>* <import>* <export>* <table>? <memory>? <global>* <elem>* <data>* <start>? )
         <typedef>* <func>* <import>* <export>* <table>? <memory>? <global>* <elem>* <data>* <start>?  ;; = 
         ( module <typedef>* <func>* <import>* <export>* <table>? <memory>? <global>* <elem>* <data>* <start>? )
```