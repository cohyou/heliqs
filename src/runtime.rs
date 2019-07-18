use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::convert::TryInto;
use core::{FuncType, Func, Mutablity, Instr, Module, ValType, ResultType, Expr};

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

struct Frame {
    locals: Vec<Val>,
    module: Rc<RefCell<ModuleInst>>,
}

struct Activation(u32, Frame);

#[derive(Default)]
pub struct Runtime {
    store: Store,
    value_stack: Vec<Val>,
    label_stack: Vec<Label>,
    frame_stack: Vec<Activation>,
}

impl Runtime {
    pub fn new(store: Option<Store>) -> Runtime {
        let mut runtime = Runtime::default();
        if let Some(store) = store {
            runtime.store = store;
        }
        runtime
    }

    pub fn instantiate(&mut self, module: &Module, extern_vals: Vec<ExternVal>) {
        // 5. Let "val^*" be the vector of <global> initialization <values> determined by "module" and "externval^n".
        // These may be calculated as follows.


        // 6. Let "moduleinst" be a new module instance <allocated> from "module" in Store S with imports "externval^n" and
        // global initializer values "val^*", and let S' be extended store produced by module allocation.
        let module_inst = self.allocate_module(module, extern_vals, vec![]);

        // 7. Let F be the <frame> { 'module' "moduleinst" 'locals' e}.
        let frame = Frame { module: module_inst.clone(), locals: vec![] };

        // 8. Push the frame F to the stack.
        self.frame_stack.push(Activation(0, frame));

        // 12. Pop the frame from the stack.
        self.frame_stack.pop();

        // 15. If the <start function> "module".'start' is not empty, then:
        if module.start.is_some() {
            // (a) Assert: due to <validation>, "moduleinst".'funcaddrs'["module".'start'.'func'] exists.
            
            // (b) Let "funcaddr" be the <function address> "moduleinst".'funcaddrs'["module".'start'.'func'].
            let func_addr = module_inst.borrow().func_addrs[module.start.clone().unwrap().func as usize];

            // (c) <Invoke> the function instance at "funcaddr".
            self.invoke_function(func_addr);
        }
    }

    fn allocate_module(&mut self, module: &Module, extern_vals: Vec<ExternVal>, vals: Vec<Val>) -> Rc<RefCell<ModuleInst>> {
        // 1. Let "module" be the <module> to allocate and "externval_im^*" the vector of <external values> providing the module's
        // imports, and "val^*" the initialization <values> of the module's <globals>.
        let module_inst = Rc::new(RefCell::new(ModuleInst::default()));

        // set types
        module_inst.borrow_mut().types = module.types.clone();

        // set funcinsts and funcaddrs
        for func in module.funcs.iter() {
            let address = self.allocate_func(func, module_inst.clone());
            module_inst.borrow_mut().func_addrs.push(address);            
        }

        // 10. Let "funcaddr_mod^*" be the list of <function addresses> extracted from "externval_im^*" concatenated with "funcaddr^*".        
        for extern_val in extern_vals {
            if let ExternVal::Func(func_addr) = extern_val {
                println!("func_addr: {:?}", func_addr);
                module_inst.borrow_mut().func_addrs.push(func_addr);
            }
        }

        module_inst
    }

    fn allocate_func(&mut self, func: &Func, module_inst: Rc<RefCell<ModuleInst>>) -> FuncAddr {
        // 1. Let "func" be the <function> to allocate "moduleinst" its <module instance>.

        // 2. Let "a" be the first free <function address> in S.
        let address = self.store.insts.len() as FuncAddr;

        // 3. Let "functype" be the <function type> "moduleinst".'types'["func".'type'].
        // let modinst = module_inst.upgrade().expect("molude instのupgradeに失敗");
        let func_type = module_inst.borrow_mut().types[func.func_type.type_index()].clone();

        // 4. Let "funcinst" be the <function instance> {'type' "functype", 'module' "moduleinst" 'code' "func"}.
        let func_inst = FuncInst::Normal { func_type: func_type.clone(), module: module_inst.clone(), code: func.clone() };

        // 5. Append "funcinst" to the 'funcs' of S.
        self.store.insts.push(StoreInst::Func(func_inst));

        // 取得したアドレスを返す
        address
    }

    fn invoke_function(&mut self, func_addr: FuncAddr) {
        // 2. Let f be the <function instance>, S.'funcs'[a].
        let f = self.store.funcs()[func_addr];

        // 3. Let [t_1^n] -> [t_2^m] be the <function type> f.'type'.
        if let FuncInst::Normal { func_type: ft, module: module_inst, code} = f {

            // 5. Let "t^*" be the list of <value types> f.'code'.'locals'.
            let local_types = &code.locals;

            // 6. Let "instr^*" be the expression f.'code'.'body'.
            let instrs = &code.body;

            // 8. Pop the values "val^n" from the stack.            
            let mut stack_values = vec![];
            for _ in local_types.iter() {
                if let Some(v) = self.value_stack.pop() {
                    stack_values.push(v);
                }                
            }
                    
            // 9. Let "val_0^*" be the list of zero values of types "t^*"
            let mut local_values = vec![];
            for local_type in local_types {
                match local_type {
                    ValType::I32 => local_values.push(Val::I32Const(0)),
                    ValType::I64 => local_values.push(Val::I64Const(0)),
                    ValType::F32 => local_values.push(Val::F32Const(0.0)),
                    ValType::F64 => local_values.push(Val::F64Const(0.0)),
                }            
            }
            
            // 10. Let F be the <frame> { 'module' f.'module', 'locals' "val^n" "val_0^*" }.
            stack_values.extend(local_values);
            let frame = Frame {
                module: module_inst.clone(),
                locals: stack_values,
            };

            // 11. Push the activation of F with arity "m" to the stack.
            let activation = Activation(ft.1.len().try_into().unwrap(), frame);
            self.frame_stack.push(activation);

            // 12. <Execute> the instruction 'block'[t_2^m] "instr^*" 'end'.
            let block_instr = Instr::Block(ft.1.clone(), instrs.clone());            
            self.execute_instr(block_instr);
            
        } else {
            panic!("普通の関数しかinvokeしません");
        }    
    }

    fn execute_instr(&mut self, instr: Instr) {
        match instr {
            Instr::Call(x) => { self.execute_call(x.try_into().unwrap()) },
            Instr::Block(result_type, instrs) => { self.execute_block(result_type, &instrs); },
            _ => {},
        }
    }

    fn execute_call(&mut self, x: FuncAddr) {
        println!("Instr::Call {:?}", x);

        // 1. Let F be the current frame.
        let current_frame = self.frame_stack.pop().unwrap();

        // 3. Let "a" be the <function address> F.'module'.'funcaddrs'[x]
        let func_addr = current_frame.1.module.borrow_mut().func_addrs[x];

        // 4. <Invoke> the function instance at address a.
        self.invoke_function(func_addr);
    }

    fn execute_block(&mut self, result_type: ResultType, expr: &Expr) {
        // 1. Let "n" be the arity |t^?| of the <result type> "t^?".
        let n = result_type.len();

        // 2. Let L be the label whose arity is "n" and whose continuation is the end of the block.
        let label = Label(n.try_into().unwrap(), vec![]);

        // 3. <Enter> the block "instr^*" with label L.
        self.enter_exprs(expr, label);
    }

    fn enter_exprs(&mut self, expr: &Expr, label: Label) {
        // 1. Push L to the stack.
        self.label_stack.push(label);

        // 2. Jump to the start of the instruction sequence <instr^*>.
        for instr in expr.instrs.iter() {
            self.execute_instr(instr.clone());
        }

        self.exit_exprs();
    }

    /// 実際にはvalueとlabelで別のstackを使っているので、1,2,5の手順は不要。一応やってる
    fn exit_exprs(&mut self) {
        // 1. Let m be the number of values on the top of the stack.
        let m = self.value_stack.len();

        // 2. Pop the values <val^m> from the stack.
        let mut values = vec![];
        for _ in 0..m {
            values.push(self.value_stack.pop().unwrap());
        }        

        // 3. Assert: due to <validation>, the label L is now on the top of the stack.
        // 4. Pop the label from the stack.
        self.label_stack.pop().expect("3. Assert: due to <validation>, the label L is now on the top of the stack.");

        // 5. Push <val^m> back to the stack.
        for v in values {
            self.value_stack.push(v);
        }

        // 6. Jump to the position after the 'end' of the <structured control instruction> associated with the label L.
    }
}