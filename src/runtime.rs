use std::rc::{Rc, Weak};
use std::cell::RefCell;
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

pub enum StoreInst {
    Func(FuncInst),
    // Table(TableInst),
    // Mem(MemInst),
    // Global(GlobalInst),
}

#[derive(Default)]
pub struct Store {
    insts: Vec<StoreInst>,
}

impl Store {
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

enum FuncInst {
    Normal { func_type: FuncType, module: Rc<RefCell<ModuleInst>>, code: Func }, // module instanceは関数で取得するようにします
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
    module: Rc<RefCell<ModuleInst>>,
}

struct Activation(u32, Frame);


pub fn instantiate(mut store: Store, module: &Module, extern_vals: Vec<ExternVal>) {
    // 6. Let "moduleinst" be a new module instance <allocated> from "module" in Store S with imports "externval^n" and
    // global initializer values "val^*", and let S' be extended store produced by module allocation.
    let module_inst = allocate_module(&mut store, module, extern_vals, vec![]);

    // 15. If the <start function> "module".'start' is not empty, then:
    if module.start.is_some() {
        // (a) Assert: due to <validation>, "moduleinst".'funcaddrs'["module".'start'.'func'] exists.
        
        // (b) Let "funcaddr" be the <function address> "moduleinst".'funcaddrs'["module".'start'.'func'].
        let func_addr = module_inst.borrow().func_addrs[module.start.clone().unwrap().func as usize];

        // (c) <Invoke> the function instance at "funcaddr".
        invoke(&store, func_addr);
    }
}

fn invoke(store: &Store, func_addr: FuncAddr) {
    // 2. Let f be the <function instance>, S.'funcs'[a].
    let f = store.funcs()[func_addr];

    // 3. Let [t_1^n] -> [t_2^m] be the <function type> f.'type'.
    if let FuncInst::Normal { func_type: ft, code: code, ..} = f {

        // 5. Let "t^*" be the list of <value types> f.'code'.'locals'.
        let locals = &code.locals;

        // 6. Let "instr^*" be the expression f.'code'.'body'.
        let instrs = &code.body;

        // 8. Pop the values "val^n" from the stack.
    
    } else {
        panic!("普通の関数しかinvokeしません");
    }
    

}

fn allocate_module(store: &mut Store, module: &Module, extern_vals: Vec<ExternVal>, vals: Vec<Val>) -> Rc<RefCell<ModuleInst>> {
    let module_inst = Rc::new(RefCell::new(ModuleInst::default()));

    // set types
    module_inst.borrow_mut().types = module.types.clone();

    // set funcinsts and funcaddrs
    for func in module.funcs.iter() {
        let address = allocate_func(store, func, module_inst.clone());
        module_inst.borrow_mut().func_addrs.push(address);            
    }

    module_inst
}

fn allocate_func(store: &mut Store, func: &Func, module_inst: Rc<RefCell<ModuleInst>>) -> FuncAddr {
    // 1. Let "func" be the <function> to allocate "moduleinst" its <module instance>.

    // 2. Let "a" be the first free <function address> in S.
    let address = store.insts.len() as FuncAddr;

    // 3. Let "functype" be the <function type> "moduleinst".'types'["func".'type'].
    // let modinst = module_inst.upgrade().expect("molude instのupgradeに失敗");
    let func_type = module_inst.borrow_mut().types[func.func_type.type_index()].clone();

    // 4. Let "funcinst" be the <function instance> {'type' "functype", 'module' "moduleinst" 'code' "func"}.
    let func_inst = FuncInst::Normal { func_type: func_type.clone(), module: module_inst.clone(), code: func.clone() };

    // 5. Append "funcinst" to the 'funcs' of S.
    store.insts.push(StoreInst::Func(func_inst));

    // 取得したアドレスを返す
    address
}
