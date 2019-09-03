#[macro_use] mod util;
mod annot;
mod instr;
mod context;
mod lexer;
mod parser;
mod mod2wasm;
mod decoder;
mod runtime;
mod error;

pub use annot::*;
pub use instr::*;
pub use lexer::*;
pub use parser::*;
pub use mod2wasm::*;
pub use runtime::*;
pub use error::Error;

pub fn module_decode() -> Result<Module, Error> {
    Err(Error::Decode)
}
