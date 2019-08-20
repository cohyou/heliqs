mod impls;

use context::*;

pub use self::impls::*;

// Function types classify the signature of functions,
// mapping a vector of parameters to a vector of results, written as follows.
// ということで、パラメータ列から結果列への写像、らしいです。
// ひとまず、vecのタプルとして持ちます。
pub type FuncType = (Vec<ValType>, Vec<ValType>);

pub type ResultType = Vec<ValType>;

// expr ::= instr* end
// expressionの長さはlimitationとして実装ごとに決定できる
// ひとまず、usizeにしておこう
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Expr(pub Vec<Instr>);


#[derive(Debug, Clone, Default, PartialEq)]
pub struct MemArg {
    align: u32,
    offset: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValType { I32, I64, F32, F64, }

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    /* Block Instructions */

    // Control Instructions
    Block(ResultType, Expr),
    Loop(ResultType, Expr),
    If(ResultType, Expr, Expr),

    /* Plain Instructions */

    // Control Instructions
    Unreachable,
    Nop,
    Br(LabelIndex),
    BrIf(LabelIndex),
    BrTable(Vec<LabelIndex>, LabelIndex),
    Return,
    Call(FuncIndex),
    CallIndirect(FuncIndex),

    // Parametric Instructions
    Drop,
    Select,

    // Variable Instructions
    GetLocal(LocalIndex),
    SetLocal(LocalIndex),
    TeeLocal(LocalIndex),
    GetGlobal(GlobalIndex),
    SetGlobal(GlobalIndex),

    // Memory Instructions
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,

    // Numeric Instructions
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),

    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32WrapToI64,
    I32TruncSToF32,
    I32TruncUToF32,
    I32TruncSToF64,
    I32TruncUToF64,
    I64ExtendSToI32,
    I64ExtendUToI32,
    I64TruncSToF32,
    I64TruncUToF32,
    I64TruncSToF64,
    I64TruncUToF64,
    F32ConvertSToI32,
    F32ConvertUToI32,
    F32ConvertSToI64,
    F32ConvertUToI64,
    F32DemoteToF64,
    F64ConvertSToI32,
    F64ConvertUToI32,
    F64ConvertSToI64,
    F64ConvertUToI64,
    F64PromoteToF32,
    I32ReinterpretToF32,
    I64ReinterpretToF64,
    F32ReinterpretToI32,
    F64ReinterpretToI64,   
}