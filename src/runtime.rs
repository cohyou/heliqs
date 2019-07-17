use core::{FuncType, Func, Mutablity, Instr, Module};

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

#[derive(Default)]
pub struct Store {
    funcs: Vec<FuncInst>,
    // tables: Vec<TableInst>,
    // mems: Vec<MemInst>,
    // globals: Vec<GlobalInst>,
}

type Addr = usize; // 仕様は自由なのでひとまずusize
type FuncAddr = Addr;
// type TableAddr = Addr;
// type MemAddr = Addr;
// type GlobalAddr = Addr;

#[derive(Debug, Default)]
pub struct ModuleInst {
    types: Vec<FuncType>,
    func_addrs: Vec<FuncAddr>,
    // table_addrs: Vec<TableAddr>,
    // mem_addrs: Vec<MemAddr>,
    // global_addrs: Vec<GlobalAddr>,
    // exports: Vec<ExportInst>,
}

type HostFunc = String; // primitiveは関数名をStringで持つことにします

enum FuncInst {
    Normal { func_type: FuncType, module: ModuleInst, code: Func},
    Host { func_type: FuncType, host_code: HostFunc },
}

// type FuncElem = Option<FuncAddr>;
// struct TableInst { elem: Vec<FuncElem>, max: Option<u32> }

// struct MemInst { data: Vec<u8>, max: Option<u32> }

// struct GlobalInst { value: Val, mutablity: Mutablity }

// struct ExportInst { name: String, value: ExternVal }

pub enum ExternVal {
    Func(FuncAddr),
    // Table(TableAddr),
    // Mem(MemAddr),
    // Global(GlobalAddr),
}

struct Label(u32, Vec<Instr>);

struct Stack {
    value_stack: Vec<Val>,
    label_stack: Vec<Label>,
    frame_stack: Vec<Activation>,
}

struct Frame {
    locals: Vec<Val>,
    module: ModuleInst,
}

struct Activation(u32, Frame);

pub fn instantiate(store: Store, module: &Module, extern_vals: Vec<ExternVal>) -> ModuleInst {
    allocate_module(store, module, extern_vals, vec![])
}

pub fn allocate_module(store: Store, module: &Module, extern_vals: Vec<ExternVal>, vals: Vec<Val>) -> ModuleInst {
    let mut module_inst = ModuleInst::default();

    module_inst.types = module.types.clone();

let num = module.funcs.len();

    module_inst.func_addrs = vec![];

    module_inst
}