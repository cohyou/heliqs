/// Instructions

use super::indices::FuncIndex;

#[derive(Debug, Clone)]
pub enum Instr {
    Call(FuncIndex),
}