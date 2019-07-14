/// Instructions

use super::indices::FuncIndex;

#[derive(Debug)]
pub enum Instr {
    Call(FuncIndex),
}