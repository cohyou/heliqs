mod impls;

use context::*;
use runtime::*;

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


#[derive(Clone, Default, PartialEq)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
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
    LocalGet(LocalIndex),
    LocalSet(LocalIndex),
    LocalTee(LocalIndex),
    GlobalGet(GlobalIndex),
    GlobalSet(GlobalIndex),

    // Memory Instructions
    Load(ValType, MemArg),
    Store(ValType, MemArg),
    ILoad8(ValSize, ValSign, MemArg),
    ILoad16(ValSize, ValSign, MemArg),
    I64Load32(ValSign, MemArg),
    IStore8(ValSize, MemArg),
    IStore16(ValSize, MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,

    // Numeric Instructions
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),

    IUnOp(ValSize, IUnOp),
    FUnOp(ValSize, FUnOp),

    IBinOp(ValSize, IBinOp),
    FBinOp(ValSize, FBinOp),

    ITestOp(ValSize, ITestOp),

    IRelOp(ValSize, IRelOp),
    FRelOp(ValSize, FRelOp),

    CvtOp(CvtOp),

    // Administrative Instructions
    Trap,
    Invoke(FuncAddr),
    InitElem(TableAddr, u32, Vec<FuncIndex>),
    InitData(MemAddr, u32, Vec<u8>),
    Label(usize, Vec<Instr>, Vec<Instr>),
    Frame(usize, Frame, Vec<Instr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValSize { V32, V64 }

#[derive(Debug, PartialEq, Clone)]
pub enum ValSign { U, S }

#[derive(Debug, Clone, PartialEq)]
pub enum IUnOp { Clz, Ctz, Popcnt, }

#[derive(Debug, Clone, PartialEq)]
pub enum IBinOp {
    Add, Sub, Mul, Div(ValSign), Rem(ValSign),
    And, Or, Xor, Shl, Shr(ValSign), Rotl, Rotr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FUnOp { Abs, Neg, Sqrt, Ceil, Floor, Trunc, Nearest, }

#[derive(Debug, Clone, PartialEq)]
pub enum FBinOp { Add, Sub, Mul, Div, Min, Max, Copysign, }

#[derive(Debug, Clone, PartialEq)]
pub enum ITestOp { Eqz, }

#[derive(Debug, Clone, PartialEq)]
pub enum IRelOp { Eq, Ne, Lt(ValSign), Gt(ValSign), Le(ValSign), Ge(ValSign), }

#[derive(Debug, Clone, PartialEq)]
pub enum FRelOp { Eq, Ne, Lt, Gt, Le, Ge, }

#[derive(Debug, Clone, PartialEq)]
pub enum CvtOp {
    I32WrapFromI64,
    I64ExtendFromI32(ValSign),
    ITruncFromF(ValSize, ValSize, ValSign),
    F32DemoteFromF64,
    F64PromoteFromF32,
    FConvertFromI(ValSize, ValSize, ValSign),
    IReinterpretFromF(ValSize),
    FReinterpretFromI(ValSize),
}