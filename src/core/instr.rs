/// Instructions

use super::indices::FuncIndex;
// use super::token::ValType;
use super::*;

#[derive(Debug, Clone)]
pub enum Val {
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),
}

impl MemArg {
    pub fn with_alignment(a: u32) -> Self {
        MemArg { align: a, offset: u32::default() }
    }
}
