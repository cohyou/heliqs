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

#[allow(dead_code)]
enum Result {
    Vals(Vec<Val>),
    Trap,
}

#[derive(Default)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
}

type Addr = usize;  // 仕様は自由なのでひとまずusize
pub type FuncAddr = Addr;
pub type TableAddr = Addr;
pub type MemAddr = Addr;
type GlobalAddr = Addr;

#[derive(Default, PartialEq)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub func_addrs: Vec<FuncAddr>,
    pub table_addrs: Vec<TableAddr>,
    pub mem_addrs: Vec<MemAddr>,
    pub global_addrs: Vec<GlobalAddr>,
    pub exports: Vec<ExportInst>,
}

type HostFunc = String; // primitiveは関数名をStringで持つことにします

pub enum FuncInst {
    Normal { func_type: FuncType, module: Rc<RefCell<ModuleInst>>, code: Func }, // module instanceは関数で取得するようにします
    Host { func_type: FuncType, host_code: HostFunc },
}

type FuncElem = Option<FuncAddr>;

#[derive(Debug)]
pub struct TableInst { elem: Vec<FuncElem>, max: Option<u32> }

#[derive(Debug)]
pub struct MemInst { data: Vec<u8>, max: Option<u32> }

#[derive(Debug)]
pub struct GlobalInst { value: Val, mutablity: Mutablity }

#[derive(Debug, PartialEq)]
pub struct ExportInst { name: String, value: ExternVal }

#[derive(Debug, PartialEq)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}

#[derive(Debug, PartialEq)]
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
