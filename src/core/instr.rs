/// Instructions

use super::indices::FuncIndex;
use super::token::ValType;
use super::*;

#[derive(Debug, Clone)]
pub enum Instr {
    Call(FuncIndex),
    Block(ResultType, Expr),
}