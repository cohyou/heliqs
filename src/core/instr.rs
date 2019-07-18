/// Instructions

use super::indices::FuncIndex;
use super::token::ValType;
use super::*;

#[derive(Debug, Clone)]
pub enum Val {
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),
}

#[derive(Debug, Clone)]
pub enum Instr {
    I32Const(Val),

    Call(FuncIndex),
    Block(ResultType, Expr),
}