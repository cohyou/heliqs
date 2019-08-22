use std::rc::Rc;
use std::cell::RefCell;

use instr::*;
use parser::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Val {
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),
}

enum Result {
    Vals(Vec<Val>),
    Trap,
}

#[derive(Debug, Default)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
}

// impl Store {
//     pub fn funcs(&self) -> Vec<&FuncInst> {
//         let mut res = vec![];
//         for inst in self.insts.iter() {
//             if let StoreInst::Func(func_inst) = inst {
//                 res.push(func_inst);
//             }
//         }
//         res
//     }
// }

type Addr = usize;  // 仕様は自由なのでひとまずusize
pub type FuncAddr = Addr;
pub type TableAddr = Addr;
pub type MemAddr = Addr;
type GlobalAddr = Addr;

#[derive(Debug, Default, PartialEq)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub func_addrs: Vec<FuncAddr>,
    table_addrs: Vec<TableAddr>,
    mem_addrs: Vec<MemAddr>,
    global_addrs: Vec<GlobalAddr>,
    exports: Vec<ExportInst>,
}

type HostFunc = String; // primitiveは関数名をStringで持つことにします

#[derive(Debug)]
pub enum FuncInst {
    Normal { func_type: FuncType, module: Rc<RefCell<ModuleInst>>, code: Func }, // module instanceは関数で取得するようにします
    Host { func_type: FuncType, host_code: HostFunc },
}

// impl FuncInst {
//     fn module_instance() -> &ModuleInst {
        
//     }
// }

type FuncElem = Option<FuncAddr>;

#[derive(Debug)]
pub struct TableInst { elem: Vec<FuncElem>, max: Option<u32> }

#[derive(Debug)]
pub struct MemInst { data: Vec<u8>, max: Option<u32> }

#[derive(Debug)]
pub struct GlobalInst { value: Val, mutablity: Mutablity }

#[derive(Debug, PartialEq)]
struct ExportInst { name: String, value: ExternVal }

#[derive(Debug, PartialEq)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}

#[derive(Debug)]
pub enum StackEntry {
    Val(Val),
    Label(usize, Vec<Instr>),
    Activation(usize, Frame),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub module: Rc<RefCell<ModuleInst>>,
}
