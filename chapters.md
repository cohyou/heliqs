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


// 4 Execution

// 4.1 Conventions

// 4.1.1 Prose Notation

// 4.1.2 Formal Notation


// 4.2 Runtime Structure

// 4.2.1 Values

// 4.2.2 Results

// 4.2.3 Store

// 4.2.4 Addresses

// 4.2.5 Module Instances

// 4.2.6 Function Instances

// 4.2.7 Table Instances

// 4.2.8 Memory Instances

// 4.2.9 Global Instances

// 4.2.10 Export Instances

// 4.2.11 External Values

// 4.2.12 Stack

// 4.2.13 Administrative Instructions


// 4.3 Numerics

// 4.3.1 Representations

// 4.3.2 Integer Operations

// 4.3.3 Floating-Point Operations

// 4.3.4 Conversions


// 4.4 Instructions

// 4.4.1 Numeric Instructions

// 4.4.2 Parametric Instructions

// 4.4.3 Variable Instructions

// 4.4.4 Memory Instructions

// 4.4.5 Control Instructions

// 4.4.6 Blocks

// 4.4.7 Function Calls

// 4.4.8 Expressions


// 4.5 Modules

// 4.5.1 External Typing

// 4.5.2 Import Matching

// 4.5.3 Allocation

// 4.5.4 Instantiation

// 4.5.5 Invocation


// 5 Binary Format

// 5.1 Conventions

// 5.1.1 Grammer

// 5.1.2 Auxiliary Notation

// 5.1.3 Vectors


// 5.2 Vectors

// 5.2.1 Bytes

// 5.2.2 Integers

// 5.2.3 Floating-Point

// 5.2.4 Names


// 5.3 Types

// 5.3.1 Value Types

// 5.3.2 Result Types

// 5.3.3 Function Types

// 5.3.4 Limits

// 5.3.5 Memory Types

// 5.3.6 Table Types

// 5.3.7 Global Types


// 5.4 Instructions

// 5.4.1 Control Instructions

// 5.4.2 Parametric Instructions

// 5.4.3 Variable Instructions

// 5.4.4 Memory Instuctions

// 5.4.5 Numeric Instuctions

// 5.4.6 Expressions


// 5.5 Modules

// 5.5.1 Indices

// 5.5.2 Sections

// 5.5.3 Custom Section

// 5.5.4 Type Section

// 5.5.5 Import Section

// 5.5.6 Function Section

// 5.5.7 Table Section

// 5.5.8 Memory Section

// 5.5.9 Global Section

// 5.5.10 Export Section

// 5.5.11 Start Section

// 5.5.12 Element Section

// 5.5.13 Code Section

// 5.5.14 Data Section

// 5.5.15 Modules


// 6 Text Format

// 6.1 Conventions

// 6.1.1 Grammar

// 6.1.2 Abbreviations

// 6.1.3 Contexts

// 6.1.4 Vectors


// 6.2 Lexical Format

// 6.2.1 Characters

// 6.2.2 Tokens

// 6.2.3 White Space

// 6.2.4 Comments


// 6.3 Values

// 6.3.1 Integers

// 6.3.2 Floating-Point

// 6.3.3 Strings

// 6.3.4 Names

// 6.3.5 Identifiers



// 6.4 Types

// 6.4.1 Value Types

// 6.4.2 Result Types

// 6.4.3 Function Types

// 6.4.4 Limits

// 6.4.5 Memory Types

// 6.4.6 Table Types

// 6.4.7 Global Types


// 6.5 Instructions

// 6.5.1 Labels

// 6.5.2 Control Instructions

// 6.5.3 Parametric Instructions

// 6.5.4 Variable Instructions

// 6.5.5 Memory Instuctions

// 6.5.6 Numeric Instuctions

// 6.5.7 Folded Instructions

// 6.5.8 Expressions


// 6.6 Modules

// 6.6.1 Indices

// 6.6.2 Types

// 6.6.3 Type Uses

// 6.6.4 Imports

// 6.6.5 Functions

// 6.6.6 Tables

// 6.6.7 Memories

// 6.6.8 Globals

// 6.6.9 Exports

// 6.6.10 Start Function

// 6.6.11 Element Segments

// 6.6.12 Data Segments

// 6.6.13 Modules


// 7 Appendix

// 7.1 Embedding

// 7.1.1 Store

// 7.1.2 Modules

// 7.1.3 Exports

// 7.1.4 Functions

// 7.1.5 Tables

// 7.1.6 Memories

// 7.1.7 Globals


// 7.2 Implementation Limitations

// 7.2.1 Syntactic Limits

// 7.2.2 Validation

// 7.2.3 Execution


// 7.3 Validation Algorithm

// 7.3.1 Data Structures

// 7.3.2 Validation of Opcode Sequences


// 7.4 Custom Sections

// 7.4.1 Name Section


// 7.5 Soundness

// 7.5.1 Values and Results

// 7.5.2 Store Validity

// 7.5.3 Configuration Validity

// 7.5.4 Administrative Intructions

// 7.5.5 Store Extension

// 7.5.6 Theorems



