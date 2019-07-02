// バイト
struct Byte(u8);

// u32 u64 s32 s64 i8 i16 i32 i64
// 以上をひとまず対象とするが、本来はバイト数は自由らしい
struct Unsigned8(u8);
struct Unsigned16(u16);
struct Unsigned32(u32);
struct Unsigned64(u64);

struct Signed32(i32);
struct Signed64(i64);

type Integer8 = Unsigned8;
type Integer16 = Unsigned16;
type Integer32 = Unsigned32;
type Integer64 = Unsigned64;

// floating-points
struct Float32(f32);
struct Float64(f64);

// Nameはcodepointの列。
// これはstrかStringか
// Due to the limitations of the binary format, the length of a name is bounded by the length of its UTF-8 encoding.

// Convention
// Code points are sometimes used interchangeably with natural numbers n < 1114112.
// 1114112はたぶんU+10FFFFのこと。

// 簡単そうなStringにします
type Name = String;


// 2.3 Types

// 2.3.1 Value Types
// valtype　::= 'i32 | 'i64 | 'f32 | 'f64
enum ValType {
    Integer32,
    Integer64,
    Float32,
    Float64,
}

// 2.3.2 Result Types
// resulttype ::= '[ valtype? '] valtypeが0個か1個の列ということ？
struct ResultType(<Option<ValType>);

// 2.3.3 Function Types
// functype ::= '[ vec(valtype) '] '-> '[ vec(valtype) ']
// これはどうやって宣言すればいいのだろう

// 2.3.4 Limits
// limits ::= '{ 'min u32, 'max u32? '}
// If no maximum is given, the respective storage can grow to any size.

// 2.3.5 Memory Types
// memtype  ::= limits

// 2.3.6 Table Types
// tabletype ::= limits elemtype
// elemtype ::= 'anyfunc

// 2.3.7 Global Types
// globaltype ::= mut valtype
// mut ::= 'const | 'var

// 2.3.8 External Types
// externtype ::= 'func functype | 'table tabletype | 'mem memtype | 'global globaltype
// ちなみに、externtypeの列から特定の種類のものだけを取り出すnotationを以下に定義する
// 順序は保存した状態で取り出す
// funcs(externtype*) = [functype | ('func functype) ∈ externtype*]
// tables(externtype*) = [tabletype | ('table tabletype) ∈ externtype*]
// mems(externtype*) = [memtype | ('mem memtype) ∈ externtype*]
// globals(externtype*) = [globaltype | ('global globaltype) ∈ externtype*]


// 2.4 Instructions

// 2.4.1 Numeric Instructions
// nn, mm ::= '32 | '64
// sx ::= 'u | 's
// instr ::= 'i nn '.const inn | 'f nn '.const fnn (例： 'i32.const 34686'とか'f64.const 3984.42')
//        | 'i nn '. iunop | 'f nn '. funop (un)
//        | 'i nn '. ibinop | 'f nn '. fbinop (bin)
//        | 'i nn '. itestop (test)
//        | 'i nn '. irelop | 'f nn '. frelop (rel)

//        | 'i32.wrap/i64 | 'i64.extend_ sx '/i32 | 'i nn '.trunc_ sx '/f mm
//        | 'f32.demote/f64 | 'f64.promote/f32 | 'f nn '.convert_ sx '/i mm

//        | 'i nn '.reinterpret/f nn | 'f nn '.reinterpret/i nn
//        | ...

// iunop ::= 'clz | 'ctz | 'popcnt
// ibinop ::= 'add | 'sub | 'mul | 'div_ sx | 'rem _ sx |
//         |  'and | 'or | 'xor | 'shl | 'shr_ sx | rotl | rotr
// funop ::= 'abs | 'neg | 'sqrt | 'ceil | 'floor | 'trunc | 'nearest
// fbinop ::= 'add | 'sub | 'mul | 'div | 'min | 'max | 'copysign
// itestop ::= 'eqz
// irelop ::= 'eq | 'ne | 'lt_ sx | 'gt_ sx | 'le_ sx | 'ge_ sx
// frelop ::= 'eq | 'ne | 'lt | 'gt | 'le | 'ge

// Convention
// unop ::= iunop | funop
// binop ::= ibinop | fbinop
// testop ::= itestop
// relop ::= irelop | frelop
// cvtop ::= 'wrap | 'extend_ sx | 'trunc_ sx | 'convert_ sx | 'demote | 'promote | 'reinterpret

// 2.4.2 Parametric Instructions
// instr ::= ...
//        | 'drop (operandを1つ捨てる)
//        | 'select (selects one of its first two operands based on whether its third operand is zero or not) 三項演算子っぽい

// 2.4.3 Vaiable Instructions
// instr ::= ...
//        | 'get_local localidx
//        | 'set_local localidx
//        | 'tee_local localidx (is like set_local but also returns its argument)
//        | 'get_global localidx
//        | 'set_global localidx

// 2.4.4 Memory Instructions
// memarg ::= {'offset u32, 'align u32}
// instr ::= ...
//        | 'i nn '.load memarg | 'f nn '.load memarg
//        | 'i nn '.store memarg | 'f nn '.store memarg
//        | 'i nn '.load8_ sx memarg | 'i nn '.load16_ sx memarg | 'i64.load32_ sx memarg
//        | 'i nn '.store8 memarg | 'i nn '.store16 memarg | 'i64.store32 memarg
//        | 'memory.size (returns current size of memory 扱うのはpage size単位)
//        | 'memory.grow (メモリを増やす、ダメなら-1を返す 扱うのはpage size単位)

// little endian
// 範囲を超えたらtrap
// 将来は64bit address
// あと、all memory instructionsで、現在はmemory index 0に暗黙的になっている。将来は変わる。

// 2.4.5 Control Instructions
// instr ::= ...
//        | 'nop
//        | 'unreachable
//        | 'block resulttype instr* 'end
//        | 'loop resulttype instr* 'end
//        | 'if resulttype instr* 'else instr* 'end
//        | 'br labelidx
//        | 'br_if labelidx
//        | 'br_table vec(labelidx) 
//        | 'return
//        | 'call funcidx
//        | 'call_indirect typeidx

// 2.4.6 Expressions
// Function bodies, iintialization values for globals, and offsets of element or data segments
// expr ::= instr* 'end

// 2.5 Modules

// 2.5.1 Indices

// 2.5.2 Types

// 2.5.3 Functions

// 2.5.4 Tables

// 2.5.5 Memories

// 2.5.6 Globals

// 2.5.7 Element Segments

// 2.5.8 Data Segments

// 2.5.9 Start Function

// 2.5.10 Exports

// 2.5.11 Imports

fn main() {
    println!("Hello, world!");
}
