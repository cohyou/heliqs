use std::fmt::Debug;

use super::annot::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
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
    Result,
    AnyFunc,
    Mutable,
    Offset,
}

#[derive(PartialEq, Clone)]
pub enum Number {
    Unsigned(usize),
}

impl Debug for Number {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           Number::Unsigned(num) => write!(f, "{:?}", num),
           _ => write!(f, "{:?}", self)
       }        
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Empty,

    Keyword(Keyword),
    Number(Number),
    String(String),
    Id(String), // $で始まる
    LeftParen,
    RightParen,
    Reserved(String),

    // Module,

    // Type,
    // Import,
    // Func,
    // Table,
    // Memory,
    // Global,
    // Export,
    // Start,
    // Elem,
    // Data,

    // Local,
    // Param,
    // FuncResult,
    // AnyFunc,
    // Mutable,
    // Offset,

    ValType(ValType),

    /* Block Instructions */

    // Control Instructions
    Block,
    Loop,
    If,

    Else,
    End,


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
    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,
    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
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

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn empty(loc: Loc) -> Self { Self::new(TokenKind::Empty, loc) }

    pub fn keyword(kw: Keyword, loc: Loc) -> Self { Self::new(TokenKind::Keyword(kw), loc) }
    pub fn number_u(num: usize, loc: Loc) -> Self { Self::new(TokenKind::Number(Number::Unsigned(num)), loc) }
    pub fn string(s: String, loc: Loc) -> Self { Self::new(TokenKind::String(s), loc) }
    pub fn id(n: String, loc: Loc) -> Self { Self::new(TokenKind::Id(n), loc) }
    pub fn left_paren(loc: Loc) -> Self { Self::new(TokenKind::LeftParen, loc) }
    pub fn right_paren(loc: Loc) -> Self { Self::new(TokenKind::RightParen, loc) }
    pub fn reserved(s: Vec<u8>, loc: Loc) -> Self { Self::new(TokenKind::Reserved(String::from_utf8(s).unwrap()), loc) }


    // pub fn module(loc: Loc) -> Self { Self::new(TokenKind::Module, loc) }

    // pub fn func_type(loc: Loc) -> Self { Self::new(TokenKind::Type, loc) }
    // pub fn import(loc: Loc) -> Self { Self::new(TokenKind::Import, loc) }
    // pub fn func(loc: Loc) -> Self { Self::new(TokenKind::Func, loc) }
    // pub fn table(loc: Loc) -> Self { Self::new(TokenKind::Table, loc) }
    // pub fn memory(loc: Loc) -> Self { Self::new(TokenKind::Memory, loc) }
    // pub fn global(loc: Loc) -> Self { Self::new(TokenKind::Global, loc) }
    // pub fn export(loc: Loc) -> Self { Self::new(TokenKind::Export, loc) }
    // pub fn start(loc: Loc) -> Self { Self::new(TokenKind::Start, loc) }
    // pub fn elem(loc: Loc) -> Self { Self::new(TokenKind::Elem, loc) }
    // pub fn data(loc: Loc) -> Self { Self::new(TokenKind::Data, loc) }

    // pub fn local(loc: Loc) -> Self { Self::new(TokenKind::Local, loc) }
    // pub fn param(loc: Loc) -> Self { Self::new(TokenKind::Param, loc) }
    // pub fn func_result(loc: Loc) -> Self { Self::new(TokenKind::FuncResult, loc) }
    // pub fn any_func(loc: Loc) -> Self { Self::new(TokenKind::AnyFunc, loc) }
    // pub fn mutable(loc: Loc) -> Self { Self::new(TokenKind::Mutable, loc) }
    // pub fn offset(loc: Loc) -> Self { Self::new(TokenKind::Offset, loc) }

    pub fn block(loc: Loc) -> Self { Self::new(TokenKind::Block, loc) }
    pub fn r#loop(loc: Loc) -> Self { Self::new(TokenKind::Loop, loc) }
    pub fn r#if(loc: Loc) -> Self { Self::new(TokenKind::If, loc) }
    pub fn r#else(loc: Loc) -> Self { Self::new(TokenKind::Else, loc) }
    pub fn end(loc: Loc) -> Self { Self::new(TokenKind::End, loc) }

    pub fn unreachable(loc: Loc) -> Self { Self::new(TokenKind::Unreachable, loc) }
    pub fn nop(loc: Loc) -> Self { Self::new(TokenKind::Nop, loc) }
    pub fn br(loc: Loc) -> Self { Self::new(TokenKind::Br, loc) }
    pub fn br_if(loc: Loc) -> Self { Self::new(TokenKind::BrIf, loc) }
    pub fn br_table(loc: Loc) -> Self { Self::new(TokenKind::BrTable, loc) }
    pub fn r#return(loc: Loc) -> Self { Self::new(TokenKind::Return, loc) }
    pub fn call(loc: Loc) -> Self { Self::new(TokenKind::Call, loc) }
    pub fn call_indirect(loc: Loc) -> Self { Self::new(TokenKind::CallIndirect, loc) }

    pub fn drop(loc: Loc) -> Self { Self::new(TokenKind::Drop, loc) }
    pub fn select(loc: Loc) -> Self { Self::new(TokenKind::Select, loc) }

    pub fn get_local(loc: Loc) -> Self { Self::new(TokenKind::GetLocal, loc) }
    pub fn set_local(loc: Loc) -> Self { Self::new(TokenKind::SetLocal, loc) }
    pub fn tee_local(loc: Loc) -> Self { Self::new(TokenKind::TeeLocal, loc) }
    pub fn get_global(loc: Loc) -> Self { Self::new(TokenKind::GetGlobal, loc) }
    pub fn set_global(loc: Loc) -> Self { Self::new(TokenKind::SetGlobal, loc) }

    pub fn i32_load(loc: Loc) -> Self { Self::new(TokenKind::I32Load, loc) }
    pub fn i64_load(loc: Loc) -> Self { Self::new(TokenKind::I64Load, loc) }
    pub fn f32_load(loc: Loc) -> Self { Self::new(TokenKind::F32Load, loc) }
    pub fn f64_load(loc: Loc) -> Self { Self::new(TokenKind::F64Load, loc) }
    pub fn i32_load8_s(loc: Loc) -> Self { Self::new(TokenKind::I32Load8S, loc) }
    pub fn i32_load8_u(loc: Loc) -> Self { Self::new(TokenKind::I32Load8U, loc) }
    pub fn i32_load16_s(loc: Loc) -> Self { Self::new(TokenKind::I32Load16S, loc) }
    pub fn i32_load16_u(loc: Loc) -> Self { Self::new(TokenKind::I32Load16U, loc) }
    pub fn i64_load8_s(loc: Loc) -> Self { Self::new(TokenKind::I64Load8S, loc) }
    pub fn i64_load8_u(loc: Loc) -> Self { Self::new(TokenKind::I64Load8U, loc) }
    pub fn i64_load16_s(loc: Loc) -> Self { Self::new(TokenKind::I64Load16S, loc) }
    pub fn i64_load16_u(loc: Loc) -> Self { Self::new(TokenKind::I64Load16U, loc) }
    pub fn i64_load32_s(loc: Loc) -> Self { Self::new(TokenKind::I64Load32S, loc) }
    pub fn i64_load32_u(loc: Loc) -> Self { Self::new(TokenKind::I64Load32U, loc) }
    pub fn i32_store(loc: Loc) -> Self { Self::new(TokenKind::I32Store, loc) }
    pub fn i64_store(loc: Loc) -> Self { Self::new(TokenKind::I64Store, loc) }
    pub fn f32_store(loc: Loc) -> Self { Self::new(TokenKind::F32Store, loc) }
    pub fn f64_store(loc: Loc) -> Self { Self::new(TokenKind::F64Store, loc) }
    pub fn i32_store8(loc: Loc) -> Self { Self::new(TokenKind::I32Store8, loc) }
    pub fn i32_store16(loc: Loc) -> Self { Self::new(TokenKind::I32Store16, loc) }
    pub fn i64_store8(loc: Loc) -> Self { Self::new(TokenKind::I64Store8, loc) }
    pub fn i64_store16(loc: Loc) -> Self { Self::new(TokenKind::I64Store16, loc) }
    pub fn i64_store32(loc: Loc) -> Self { Self::new(TokenKind::I64Store32, loc) }
    pub fn memory_size(loc: Loc) -> Self { Self::new(TokenKind::MemorySize, loc) }
    pub fn memory_grow(loc: Loc) -> Self { Self::new(TokenKind::MemoryGrow, loc) }

    pub fn i32_const(loc: Loc) -> Self { Self::new(TokenKind::I32Const, loc) }
    pub fn i64_const(loc: Loc) -> Self { Self::new(TokenKind::I64Const, loc) }
    pub fn f32_const(loc: Loc) -> Self { Self::new(TokenKind::F32Const, loc) }
    pub fn f64_const(loc: Loc) -> Self { Self::new(TokenKind::F64Const, loc) }

    pub fn i32_clz(loc: Loc) -> Self { Self::new(TokenKind::I32Clz, loc) }
    pub fn i32_ctz(loc: Loc) -> Self { Self::new(TokenKind::I32Ctz, loc) }
    pub fn i32_popcnt(loc: Loc) -> Self { Self::new(TokenKind::I32Popcnt, loc) }
    pub fn i32_add(loc: Loc) -> Self { Self::new(TokenKind::I32Add, loc) }
    pub fn i32_sub(loc: Loc) -> Self { Self::new(TokenKind::I32Sub, loc) }
    pub fn i32_mul(loc: Loc) -> Self { Self::new(TokenKind::I32Mul, loc) }
    pub fn i32_div_s(loc: Loc) -> Self { Self::new(TokenKind::I32DivS, loc) }
    pub fn i32_div_u(loc: Loc) -> Self { Self::new(TokenKind::I32DivU, loc) }
    pub fn i32_rem_s(loc: Loc) -> Self { Self::new(TokenKind::I32RemS, loc) }
    pub fn i32_rem_u(loc: Loc) -> Self { Self::new(TokenKind::I32RemU, loc) }
    pub fn i32_and(loc: Loc) -> Self { Self::new(TokenKind::I32And, loc) }
    pub fn i32_or(loc: Loc) -> Self { Self::new(TokenKind::I32Or, loc) }
    pub fn i32_xor(loc: Loc) -> Self { Self::new(TokenKind::I32Xor, loc) }
    pub fn i32_shl(loc: Loc) -> Self { Self::new(TokenKind::I32Shl, loc) }
    pub fn i32_shr_s(loc: Loc) -> Self { Self::new(TokenKind::I32ShrS, loc) }
    pub fn i32_shr_u(loc: Loc) -> Self { Self::new(TokenKind::I32ShrU, loc) }
    pub fn i32_rotl(loc: Loc) -> Self { Self::new(TokenKind::I32Rotl, loc) }
    pub fn i32_rotr(loc: Loc) -> Self { Self::new(TokenKind::I32Rotr, loc) }

    pub fn i64_clz(loc: Loc) -> Self { Self::new(TokenKind::I64Clz, loc) }
    pub fn i64_ctz(loc: Loc) -> Self { Self::new(TokenKind::I64Ctz, loc) }
    pub fn i64_popcnt(loc: Loc) -> Self { Self::new(TokenKind::I64Popcnt, loc) }
    pub fn i64_add(loc: Loc) -> Self { Self::new(TokenKind::I64Add, loc) }
    pub fn i64_sub(loc: Loc) -> Self { Self::new(TokenKind::I64Sub, loc) }
    pub fn i64_mul(loc: Loc) -> Self { Self::new(TokenKind::I64Mul, loc) }
    pub fn i64_div_s(loc: Loc) -> Self { Self::new(TokenKind::I64DivS, loc) }
    pub fn i64_div_u(loc: Loc) -> Self { Self::new(TokenKind::I64DivU, loc) }
    pub fn i64_rem_s(loc: Loc) -> Self { Self::new(TokenKind::I64RemS, loc) }
    pub fn i64_rem_u(loc: Loc) -> Self { Self::new(TokenKind::I64RemU, loc) }
    pub fn i64_and(loc: Loc) -> Self { Self::new(TokenKind::I64And, loc) }
    pub fn i64_or(loc: Loc) -> Self { Self::new(TokenKind::I64Or, loc) }
    pub fn i64_xor(loc: Loc) -> Self { Self::new(TokenKind::I64Xor, loc) }
    pub fn i64_shl(loc: Loc) -> Self { Self::new(TokenKind::I64Shl, loc) }
    pub fn i64_shr_s(loc: Loc) -> Self { Self::new(TokenKind::I64ShrS, loc) }
    pub fn i64_shr_u(loc: Loc) -> Self { Self::new(TokenKind::I64ShrU, loc) }
    pub fn i64_rotl(loc: Loc) -> Self { Self::new(TokenKind::I64Rotl, loc) }
    pub fn i64_rotr(loc: Loc) -> Self { Self::new(TokenKind::I64Rotr, loc) }

    pub fn f32_abs(loc: Loc) -> Self { Self::new(TokenKind::F32Abs, loc) }
    pub fn f32_neg(loc: Loc) -> Self { Self::new(TokenKind::F32Neg, loc) }
    pub fn f32_ceil(loc: Loc) -> Self { Self::new(TokenKind::F32Ceil, loc) }
    pub fn f32_floor(loc: Loc) -> Self { Self::new(TokenKind::F32Floor, loc) }
    pub fn f32_trunc(loc: Loc) -> Self { Self::new(TokenKind::F32Trunc, loc) }
    pub fn f32_nearest(loc: Loc) -> Self { Self::new(TokenKind::F32Nearest, loc) }
    pub fn f32_sqrt(loc: Loc) -> Self { Self::new(TokenKind::F32Sqrt, loc) }
    pub fn f32_add(loc: Loc) -> Self { Self::new(TokenKind::F32Add, loc) }
    pub fn f32_sub(loc: Loc) -> Self { Self::new(TokenKind::F32Sub, loc) }
    pub fn f32_mul(loc: Loc) -> Self { Self::new(TokenKind::F32Mul, loc) }
    pub fn f32_div(loc: Loc) -> Self { Self::new(TokenKind::F32Div, loc) }
    pub fn f32_min(loc: Loc) -> Self { Self::new(TokenKind::F32Min, loc) }
    pub fn f32_max(loc: Loc) -> Self { Self::new(TokenKind::F32Max, loc) }
    pub fn f32_copysign(loc: Loc) -> Self { Self::new(TokenKind::F32Copysign, loc) }

    pub fn f64_abs(loc: Loc) -> Self { Self::new(TokenKind::F64Abs, loc) }
    pub fn f64_neg(loc: Loc) -> Self { Self::new(TokenKind::F64Neg, loc) }
    pub fn f64_ceil(loc: Loc) -> Self { Self::new(TokenKind::F64Ceil, loc) }
    pub fn f64_floor(loc: Loc) -> Self { Self::new(TokenKind::F64Floor, loc) }
    pub fn f64_trunc(loc: Loc) -> Self { Self::new(TokenKind::F64Trunc, loc) }
    pub fn f64_nearest(loc: Loc) -> Self { Self::new(TokenKind::F64Nearest, loc) }
    pub fn f64_sqrt(loc: Loc) -> Self { Self::new(TokenKind::F64Sqrt, loc) }
    pub fn f64_add(loc: Loc) -> Self { Self::new(TokenKind::F64Add, loc) }
    pub fn f64_sub(loc: Loc) -> Self { Self::new(TokenKind::F64Sub, loc) }
    pub fn f64_mul(loc: Loc) -> Self { Self::new(TokenKind::F64Mul, loc) }
    pub fn f64_div(loc: Loc) -> Self { Self::new(TokenKind::F64Div, loc) }
    pub fn f64_min(loc: Loc) -> Self { Self::new(TokenKind::F64Min, loc) }
    pub fn f64_max(loc: Loc) -> Self { Self::new(TokenKind::F64Max, loc) }
    pub fn f64_copysign(loc: Loc) -> Self { Self::new(TokenKind::F64Copysign, loc) }

    pub fn i32_eqz(loc: Loc) -> Self { Self::new(TokenKind::I32Eqz, loc) }
    pub fn i32_eq(loc: Loc) -> Self { Self::new(TokenKind::I32Eq, loc) }
    pub fn i32_ne(loc: Loc) -> Self { Self::new(TokenKind::I32Ne, loc) }
    pub fn i32_lt_s(loc: Loc) -> Self { Self::new(TokenKind::I32LtS, loc) }
    pub fn i32_lt_u(loc: Loc) -> Self { Self::new(TokenKind::I32LtU, loc) }
    pub fn i32_gt_s(loc: Loc) -> Self { Self::new(TokenKind::I32GtS, loc) }
    pub fn i32_gt_u(loc: Loc) -> Self { Self::new(TokenKind::I32GtU, loc) }
    pub fn i32_le_s(loc: Loc) -> Self { Self::new(TokenKind::I32LeS, loc) }
    pub fn i32_le_u(loc: Loc) -> Self { Self::new(TokenKind::I32LeU, loc) }
    pub fn i32_ge_s(loc: Loc) -> Self { Self::new(TokenKind::I32GeS, loc) }
    pub fn i32_ge_u(loc: Loc) -> Self { Self::new(TokenKind::I32GeU, loc) }

    pub fn i64_eqz(loc: Loc) -> Self { Self::new(TokenKind::I64Eqz, loc) }
    pub fn i64_eq(loc: Loc) -> Self { Self::new(TokenKind::I64Eq, loc) }
    pub fn i64_ne(loc: Loc) -> Self { Self::new(TokenKind::I64Ne, loc) }
    pub fn i64_lt_s(loc: Loc) -> Self { Self::new(TokenKind::I64LtS, loc) }
    pub fn i64_lt_u(loc: Loc) -> Self { Self::new(TokenKind::I64LtU, loc) }
    pub fn i64_gt_s(loc: Loc) -> Self { Self::new(TokenKind::I64GtS, loc) }
    pub fn i64_gt_u(loc: Loc) -> Self { Self::new(TokenKind::I64GtU, loc) }
    pub fn i64_le_s(loc: Loc) -> Self { Self::new(TokenKind::I64LeS, loc) }
    pub fn i64_le_u(loc: Loc) -> Self { Self::new(TokenKind::I64LeU, loc) }
    pub fn i64_ge_s(loc: Loc) -> Self { Self::new(TokenKind::I64GeS, loc) }
    pub fn i64_ge_u(loc: Loc) -> Self { Self::new(TokenKind::I64GeU, loc) }

    pub fn f32_eq(loc: Loc) -> Self { Self::new(TokenKind::F32Eq, loc) }
    pub fn f32_ne(loc: Loc) -> Self { Self::new(TokenKind::F32Ne, loc) }
    pub fn f32_lt(loc: Loc) -> Self { Self::new(TokenKind::F32Lt, loc) }
    pub fn f32_gt(loc: Loc) -> Self { Self::new(TokenKind::F32Gt, loc) }
    pub fn f32_le(loc: Loc) -> Self { Self::new(TokenKind::F32Le, loc) }
    pub fn f32_ge(loc: Loc) -> Self { Self::new(TokenKind::F32Ge, loc) }

    pub fn f64_eq(loc: Loc) -> Self { Self::new(TokenKind::F64Eq, loc) }
    pub fn f64_ne(loc: Loc) -> Self { Self::new(TokenKind::F64Ne, loc) }
    pub fn f64_lt(loc: Loc) -> Self { Self::new(TokenKind::F64Lt, loc) }
    pub fn f64_gt(loc: Loc) -> Self { Self::new(TokenKind::F64Gt, loc) }
    pub fn f64_le(loc: Loc) -> Self { Self::new(TokenKind::F64Le, loc) }
    pub fn f64_ge(loc: Loc) -> Self { Self::new(TokenKind::F64Ge, loc) }

    pub fn i32_wrap_to_i64(loc: Loc) -> Self { Self::new(TokenKind::I32WrapToI64, loc) }
    pub fn i32_trunc_s_to_f32(loc: Loc) -> Self { Self::new(TokenKind::I32TruncSToF32, loc) }
    pub fn i32_trunc_u_to_f32(loc: Loc) -> Self { Self::new(TokenKind::I32TruncUToF32, loc) }
    pub fn i32_trunc_s_to_f64(loc: Loc) -> Self { Self::new(TokenKind::I32TruncSToF64, loc) }
    pub fn i32_trunc_u_to_f64(loc: Loc) -> Self { Self::new(TokenKind::I32TruncUToF64, loc) }
    pub fn i64_extend_s_to_i32(loc: Loc) -> Self { Self::new(TokenKind::I64ExtendSToI32, loc) }
    pub fn i64_extend_u_to_i32(loc: Loc) -> Self { Self::new(TokenKind::I64ExtendUToI32, loc) }
    pub fn i64_trunc_s_to_f32(loc: Loc) -> Self { Self::new(TokenKind::I64TruncSToF32, loc) }
    pub fn i64_trunc_u_to_f32(loc: Loc) -> Self { Self::new(TokenKind::I64TruncUToF32, loc) }
    pub fn i64_trunc_s_to_f64(loc: Loc) -> Self { Self::new(TokenKind::I64TruncSToF64, loc) }
    pub fn i64_trunc_u_to_f64(loc: Loc) -> Self { Self::new(TokenKind::I64TruncUToF64, loc) }
    pub fn f32_convert_s_to_i32(loc: Loc) -> Self { Self::new(TokenKind::F32ConvertSToI32, loc) }
    pub fn f32_convert_u_to_i32(loc: Loc) -> Self { Self::new(TokenKind::F32ConvertUToI32, loc) }
    pub fn f32_convert_s_to_i64(loc: Loc) -> Self { Self::new(TokenKind::F32ConvertSToI64, loc) }
    pub fn f32_convert_u_to_i64(loc: Loc) -> Self { Self::new(TokenKind::F32ConvertUToI64, loc) }
    pub fn f32_demote_to_f64(loc: Loc) -> Self { Self::new(TokenKind::F32DemoteToF64, loc) }
    pub fn f64_convert_s_to_i32(loc: Loc) -> Self { Self::new(TokenKind::F64ConvertSToI32, loc) }
    pub fn f64_convert_u_to_i32(loc: Loc) -> Self { Self::new(TokenKind::F64ConvertUToI32, loc) }
    pub fn f64_convert_s_to_i64(loc: Loc) -> Self { Self::new(TokenKind::F64ConvertSToI64, loc) }
    pub fn f64_convert_u_to_i64(loc: Loc) -> Self { Self::new(TokenKind::F64ConvertUToI64, loc) }
    pub fn f64_promote_to_f32(loc: Loc) -> Self { Self::new(TokenKind::F64PromoteToF32, loc) }
    pub fn i32_reinterpret_to_f32(loc: Loc) -> Self { Self::new(TokenKind::I32ReinterpretToF32, loc) }
    pub fn i64_reinterpret_to_f64(loc: Loc) -> Self { Self::new(TokenKind::I64ReinterpretToF64, loc) }
    pub fn f32_reinterpret_to_i32(loc: Loc) -> Self { Self::new(TokenKind::F32ReinterpretToI32, loc) }
    pub fn f64_reinterpret_to_i64(loc: Loc) -> Self { Self::new(TokenKind::F64ReinterpretToI64, loc) }

    pub fn val_type(vt: ValType, loc: Loc) -> Self { Self::new(TokenKind::ValType(vt), loc) }    
}

impl Debug for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self.value {
           TokenKind::Keyword(kw) => write!(f, "{:?}<{:?}>", kw, self.loc),
           TokenKind::Number(num) => write!(f, "{:?}<{:?}>", num, self.loc),
           TokenKind::String(s) => write!(f, "{:?}<{:?}>", s, self.loc),
           TokenKind::Id(id) => write!(f, "${}<{:?}>", id, self.loc),
           _ => write!(f, "{:?}<{:?}>", self.value, self.loc)
       }        
    }
}