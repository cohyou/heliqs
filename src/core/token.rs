use super::annot::*;

// valtype ::= i32 | i64 | f32 | f64
#[derive(Debug, PartialEq, Clone)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

impl Default for ValType {
    fn default() -> Self {
        ValType::I32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,

    Module,

    Type,
    Import,
    Func,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Elem,
    Data,

    Local,
    Param,
    FuncResult,
    AnyFunc,
    Mutable,
    Offset,


    /* Block Instructions */

    // Control Instructions
    Block,
    Loop,
    If,


    /* Plain Instructions */

    // Control Instructions
    Unreachable,
    Nop,
    Br,
    BrIf,
    BrTable,
    Return,
    Call,
    CallIndirect,

    // Parametric Instructions
    Drop,
    Select,

    // Variable Instructions
    GetLocal,
    SetLocal,
    TeeLocal,
    GetGlobal,
    SetGlobal,

    // Memory Instructions
    I32Load,
    I64Load,
    F32Load,
    F64Load,
    I32Load8s,
    I32Load8u,
    I32Load16s,
    I32Load16u,
    I64Load8s,
    I64Load8u,
    I64Load16s,
    I64Load16u,
    I64Load32s,
    I64Load32u,
    I32Store,
    I64Store,
    F32Store,
    F64Store,
    I32Store8,
    I32Store16,
    I64Store8,
    I64Store16,
    I64Store32,
    MemorySize,
    MemoryGrow,

    // Numeric Instructions
    I32Const,
    I64Const,
    F32Const,
    F64Const,

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
    F32CopySign,

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
    F64CopySign,

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


    ValType(ValType),
    Symbol(String),
    Id(String), // $で始まる
    Text(String), // 普通の文字列 "で囲まれている
    End,
    Empty,
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn left_paren(loc: Loc) -> Self { Self::new(TokenKind::LeftParen, loc) }
    pub fn right_paren(loc: Loc) -> Self { Self::new(TokenKind::RightParen, loc) }

    pub fn module(loc: Loc) -> Self { Self::new(TokenKind::Module, loc) }

    pub fn func_type(loc: Loc) -> Self { Self::new(TokenKind::Type, loc) }
    pub fn import(loc: Loc) -> Self { Self::new(TokenKind::Import, loc) }
    pub fn func(loc: Loc) -> Self { Self::new(TokenKind::Func, loc) }
    pub fn table(loc: Loc) -> Self { Self::new(TokenKind::Table, loc) }
    pub fn memory(loc: Loc) -> Self { Self::new(TokenKind::Memory, loc) }
    pub fn global(loc: Loc) -> Self { Self::new(TokenKind::Global, loc) }
    pub fn export(loc: Loc) -> Self { Self::new(TokenKind::Export, loc) }
    pub fn start(loc: Loc) -> Self { Self::new(TokenKind::Start, loc) }
    pub fn elem(loc: Loc) -> Self { Self::new(TokenKind::Elem, loc) }
    pub fn data(loc: Loc) -> Self { Self::new(TokenKind::Data, loc) }

    pub fn local(loc: Loc) -> Self { Self::new(TokenKind::Local, loc) }
    pub fn param(loc: Loc) -> Self { Self::new(TokenKind::Param, loc) }
    pub fn func_result(loc: Loc) -> Self { Self::new(TokenKind::FuncResult, loc) }
    pub fn any_func(loc: Loc) -> Self { Self::new(TokenKind::AnyFunc, loc) }
    pub fn mutable(loc: Loc) -> Self { Self::new(TokenKind::Mutable, loc) }
    pub fn offset(loc: Loc) -> Self { Self::new(TokenKind::Offset, loc) }

    pub fn call(loc: Loc) -> Self { Self::new(TokenKind::Call, loc) }

    pub fn i32_const(loc: Loc) -> Self { Self::new(TokenKind::I32Const, loc) }

    pub fn val_type(vt: ValType, loc: Loc) -> Self { Self::new(TokenKind::ValType(vt), loc) }
    pub fn symbol(s: String, loc: Loc) -> Self { Self::new(TokenKind::Symbol(s), loc) }
    pub fn id(n: String, loc: Loc) -> Self { Self::new(TokenKind::Id(n), loc) }
    pub fn text(t: String, loc: Loc) -> Self { Self::new(TokenKind::Text(t), loc) }
    pub fn end(loc: Loc) -> Self { Self::new(TokenKind::End, loc) }
    pub fn empty(loc: Loc) -> Self { Self::new(TokenKind::Empty, loc) }
}