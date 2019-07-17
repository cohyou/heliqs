use std::rc::{Rc, Weak};
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

pub enum StoreInst<'a> {
    Func(FuncInst<'a>),
    // Table(TableInst),
    // Mem(MemInst),
    // Global(GlobalInst),
}

#[derive(Default)]
pub struct Store<'a> {
    insts: Vec<StoreInst<'a>>,
}

impl<'a> Store<'a> {
    fn funcs(&self) -> Vec<&FuncInst> {
        let mut res = vec![];
        for inst in self.insts.iter() {
            if let StoreInst::Func(func_inst) = inst {
                res.push(func_inst);
            }
        }
        res
    }
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

enum FuncInst<'a> {
    Normal { func_type: FuncType, module: &'a ModuleInst, code: Func }, // module instanceは関数で取得するようにします
    Host { func_type: FuncType, host_code: HostFunc },
}

// impl FuncInst {
//     fn module_instance() -> &ModuleInst {
        
//     }
// }

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

struct Runtime<'a> {
    store: Store<'a>,    
    module_inst: ModuleInst,
}

impl<'a> Runtime<'a> {
    fn new(module: &Module, store: Store<'a>) -> Runtime<'a> {
        let mut module_inst = ModuleInst::default();

        module_inst.types = module.types.clone();

        for func in module.funcs.iter() {
            let address = allocate_func(&mut store, func, &module_inst);
            module_inst.func_addrs.push(address);            
        }

        Runtime {
            store: store,            
            module_inst: module_inst,
        }
    }

    fn instantiate(&mut self, module: &Module) {
        // let mut module_inst = ModuleInst::default();

        

        // self.module_inst = module_inst;
    }

    // fn allocate_func(&mut self, func: &Func) -> (FuncAddr, FuncInst) {
    //     // 1. Let "func" be the <function> to allocate "moduleinst" its <module instance>.

    //     // 2. Let "a" be the first free <function address> in S.
    //     let address = self.store.insts.len() as FuncAddr;

    //     // 3. Let "functype" be the <function type> "moduleinst".'types'["func".'type'].
    //     let func_type = &self.module_inst.types[func.func_type.type_index()];

    //     // 4. Let "funcinst" be the <function instance> {'type' "functype", 'module' "moduleinst" 'code' "func"}.
    //     let func_inst = FuncInst::Normal { func_type: func_type.clone(), module: &self.module_inst, code: func.clone() };

    //     (address, func_inst)
    // }
}

pub fn instantiate(store: Store, module: &Module, extern_vals: Vec<ExternVal>) -> ModuleInst {
    // allocate_module(store, module, extern_vals, vec![])    

    let runtime = Runtime::new(module, store);
    runtime.module_inst
}

// pub fn allocate_module<'a>(mut store: Store, module: &Module, extern_vals: Vec<ExternVal>, vals: Vec<Val>) -> ModuleInst {
//     let mut module_inst = ModuleInst::default();

//     // set types
//     module_inst.types = module.types.clone();


//     // let rc = Rc::new(&module_inst);
//     // let weak = Rc::downgrade(&rc);

//     // set funcinsts and funcaddrs
//     for func in module.funcs {        
//         module_inst.func_addrs.push(allocate_func(&mut store, func, module_inst));
//     }

//     module_inst
// }

fn allocate_func<'a>(store: &mut Store<'a>, func: &Func, module_inst: &'a ModuleInst) -> FuncAddr {
    // 1. Let "func" be the <function> to allocate "moduleinst" its <module instance>.

    // 2. Let "a" be the first free <function address> in S.
    let address = store.insts.len() as FuncAddr;

    // 3. Let "functype" be the <function type> "moduleinst".'types'["func".'type'].
    // let modinst = module_inst.upgrade().expect("molude instのupgradeに失敗");
    let func_type = &module_inst.types[func.func_type.type_index()];

    // 4. Let "funcinst" be the <function instance> {'type' "functype", 'module' "moduleinst" 'code' "func"}.
    let func_inst = FuncInst::Normal { func_type: func_type.clone(), module: module_inst, code: func.clone() };

    // 5. Append "funcinst" to the 'funcs' of S.
    store.insts.push(StoreInst::Func(func_inst));

    // 取得したアドレスを返す
    address
}
