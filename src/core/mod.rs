mod annot;
mod token;
mod cst;
mod indices;
mod instr;

pub use self::annot::*;
pub use self::token::*;
pub use self::cst::*;
pub use self::indices::*;
pub use self::instr::*;

impl Func {
    pub fn new() -> Func {
        Func { func_type: TypeIndex::default(), locals: vec![], body: Expr { instrs: vec![] } }
    }
}




impl Table {
    pub fn new() -> Table {
        Table { table_type:
                TableType {
                    limits: Limits { min: 0, max: None },
                    elem_type: ElemType::AnyFunc
                }
        }
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory_type: MemType { limits: Limits { min: 0, max: None } } }
    }
}

impl Global {
    pub fn new() -> Global {
        Global {
            global_type: GlobalType::default(),
            init: Expr::default(),
        }
    }
}
