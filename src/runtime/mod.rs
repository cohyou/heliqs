mod structure;

use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;

use instr::*;
use parser::*;

pub use self::structure::*;

#[derive(Default)]
pub struct Runtime {
    pub store: Store,
    pub stack: Vec<StackEntry>,
}

impl Runtime {
    pub fn init_store() -> Store {
        Store::default()
    }

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
        self.stack.push(StackEntry::Activation(0, frame));

        // 12. Pop the frame from the stack.
        self.stack.pop();

        // 15. If the <start function> "module".'start' is not empty, then:
        if module.start.is_some() {
            // (a) Assert: due to <validation>, "moduleinst".'funcaddrs'["module".'start'.'func'] exists.

            // (b) Let "funcaddr" be the <function address> "moduleinst".'funcaddrs'["module".'start'.'func'].
            let func_addr = module_inst.borrow().func_addrs[module.start.clone().unwrap().0 as usize];

            // (c) <Invoke> the function instance at "funcaddr".
            self.invoke_function(func_addr);
        }
    }

    fn allocate_module(&mut self, module: &Module, extern_vals: Vec<ExternVal>, _vals: Vec<Val>) -> Rc<RefCell<ModuleInst>> {
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
        let mut func_addr_idx = 0;
        for extern_val in extern_vals {
            if let ExternVal::Func(func_addr) = extern_val {
                // println!("func_addr: {:?}", func_addr);

                module_inst.borrow_mut().func_addrs.insert(func_addr_idx, func_addr);
                func_addr_idx += 1;
            }
        }

        module_inst
    }

    fn allocate_func(&mut self, func: &Func, module_inst: Rc<RefCell<ModuleInst>>) -> FuncAddr {
        // 1. Let "func" be the <function> to allocate "moduleinst" its <module instance>.

        // 2. Let "a" be the first free <function address> in S.
        let address = self.store.funcs.len() as FuncAddr;

        // 3. Let "functype" be the <function type> "moduleinst".'types'["func".'type'].
        // let modinst = module_inst.upgrade().expect("molude instのupgradeに失敗");
        let func_type = module_inst.borrow_mut().types[func.0 as usize].clone();

        // 4. Let "funcinst" be the <function instance> {'type' "functype", 'module' "moduleinst" 'code' "func"}.
        let func_inst = FuncInst::Normal { func_type: func_type.clone(), module: module_inst.clone(), code: func.clone() };

        // 5. Append "funcinst" to the 'funcs' of S.
        self.store.funcs.push(func_inst);

        // 取得したアドレスを返す
        address
    }

    fn invoke_function(&mut self, func_addr: FuncAddr) {
        // 2. Let f be the <function instance>, S.'funcs'[a].
        let f = &self.store.funcs[func_addr];

        // 3. Let [t_1^n] -> [t_2^m] be the <function type> f.'type'.
        // println!("invoke_function f: {:?}", f);
        match f {
            FuncInst::Normal { func_type: ft, module: module_inst, code} => {

                // 5. Let "t^*" be the list of <value types> f.'code'.'locals'.
                let local_types = &code.1;

                // 6. Let "instr^*" be the expression f.'code'.'body'.
                let instrs = &code.2;

                // 8. Pop the values "val^n" from the stack.
                let mut stack_values = vec![];
                for _ in local_types.iter() {
                    if let Some(StackEntry::Val(v)) = self.stack.pop() {
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
                let activation = StackEntry::Activation(ft.1.len().try_into().unwrap(), frame);
                self.stack.push(activation);

                // 12. <Execute> the instruction 'block'[t_2^m] "instr^*" 'end'.
                let block_instr = Instr::Block(ft.1.clone(), instrs.clone());
                self.execute_instr(block_instr);
            },

            FuncInst::Host { func_type: _ft, host_code } => {
                match host_code.as_ref() {
                    "log" => {
                        // println!("host function func_type: {:?}", ft);
                        p!(self.store);
                        println!("host function invoked! {:?}", self.stack);
                    },
                    _ => {},
                }
                
            },
        }

        self.return_from_function();
    }

    fn execute_instr(&mut self, instr: Instr) {
        match instr {            
            Instr::Call(x) => { self.execute_call(x.try_into().unwrap()); },
            Instr::Block(result_type, instrs) => { self.execute_block(result_type, &instrs); },
            Instr::I32Const(val) => { self.stack.push(StackEntry::Val(Val::I32Const(val))); }
            Instr::IBinOp(ValSize::V32, ibinop) => { self.execute_ibinop(&ibinop); }
            _ => {},
        }
    }

    fn execute_ibinop(&mut self, ibinop: &IBinOp) {
        if let StackEntry::Val(Val::I32Const(c2)) = self.stack.pop().unwrap() {
            if let StackEntry::Val(Val::I32Const(c1)) = self.stack.pop().unwrap() {
                let val = 
                match ibinop {
                    IBinOp::Add => Val::I32Const(c1 + c2),
                    IBinOp::Sub => Val::I32Const(c1 - c2),
                    IBinOp::Mul => Val::I32Const(c1 * c2),
                    IBinOp::Div(_) => Val::I32Const(c1 / c2),
                    IBinOp::Rem(_) => Val::I32Const(c1 % c2),
                    _ => unimplemented!(),
                };
                self.stack.push(StackEntry::Val(val))
            }
        }
    }

    fn execute_call(&mut self, x: FuncAddr) {
        // 1. Let F be the current frame.
        let (_, current_frame) = self.get_current_frame();

        // 3. Let "a" be the <function address> F.'module'.'funcaddrs'[x]
        let func_addr = current_frame.module.borrow_mut().func_addrs[x];

        // 4. <Invoke> the function instance at address a.
        self.invoke_function(func_addr);
    }

    fn execute_block(&mut self, result_type: ResultType, expr: &Expr) {
        // 1. Let "n" be the arity |t^?| of the <result type> "t^?".
        let n = result_type.len();

        // 2. Let L be the label whose arity is "n" and whose continuation is the end of the block.
        let label = StackEntry::Label(n.try_into().unwrap(), vec![]);

        // 3. <Enter> the block "instr^*" with label L.
        self.enter_exprs(expr, label);
    }

    fn enter_exprs(&mut self, expr: &Expr, label: StackEntry) {

        // 1. Push L to the stack.
        self.stack.push(label);

        // 2. Jump to the start of the instruction sequence <instr^*>.
        for instr in expr.0.iter() {
            // println!("enter_exprs: {:?}", instr);
            self.execute_instr(instr.clone());
        }

        self.exit_exprs();
    }

    // #[allow(dead_code)]
    fn exit_exprs(&mut self) {
        // 1. Let m be the number of values on the top of the stack.
        let mut stack_iter = self.stack.iter();
        let pos = stack_iter.rposition(|entry| self.is_stack_entry_value(entry) ).unwrap_or(0);
        let m = self.stack.len() - pos;

        // 2. Pop the values <val^m> from the stack.
        let mut values = vec![];
        for _ in 0..m {
            values.push(self.stack.pop().unwrap());
        }

        // 3. Assert: due to <validation>, the label L is now on the top of the stack.
        // 4. Pop the label from the stack.
        self.stack.pop().expect("3. Assert: due to <validation>, the label L is now on the top of the stack.");

        // 5. Push <val^m> back to the stack.
        for v in values {
            self.stack.push(v);
        }

        // 6. Jump to the position after the 'end' of the <structured control instruction> associated with the label L.
    }

    fn return_from_function(&mut self) {
        // 1. Let F be the current frame.
        // 2. Let n be the arity of the activation of F.
        let (&n, _) = self.get_current_frame();

        // 4. Pop the results <val^n> from the stack.
        let mut values = vec![];
        for _ in 0..n {
            values.push(self.stack.pop().unwrap());
        }

        // 5. Assert: due to <validation>, the frame F is now on the top of the stack.
        // 6. Pop the frame from the stack.
        self.stack.pop().expect("5. Assert: due to <validation>, the frame F is now on the top of the stack.");

        // 7. Push <val^n> back to the stack.
        for v in values {
            self.stack.push(v);
        }

        // 8. Jump to the instruction after the original call.
    }

    fn get_current_frame(&self) -> (&usize, &Frame) {
        let mut stack_iter = self.stack.iter();
        let activation = stack_iter.rfind(|entry| self.is_stack_entry_activation(entry));
        p!(self.stack);
        if let Some(StackEntry::Activation(arity, frame)) = activation {
            (arity, frame)
        } else {
            panic!("get_current_frame");
        }
    }

    fn is_stack_entry_value(&self, entry: &StackEntry) -> bool {
        if let StackEntry::Val(_) = entry { true } else { false }
    }

    fn is_stack_entry_activation(&self, entry: &StackEntry) -> bool {
        if let StackEntry::Activation(_,  _) = entry { true } else { false }
    }
}

use std::fmt::Debug;
use self::structure::ModuleInst;

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        if self.funcs.len() > 0 {
            writeln!(f, " funcs:")?;
            for func in &self.funcs {
                writeln!(f, "   {:?}", func)?;
            }            
        }
        if self.tables.len() > 0 { writeln!(f, " tables: {:?}", self.tables)?; }
        if self.mems.len() > 0 { writeln!(f, " mems: {:?}", self.mems)?; }
        if self.globals.len() > 0 { writeln!(f, " globals: {:?}", self.globals)?; }
        writeln!(f, " }}")
    }
}

impl Debug for ModuleInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        if self.types.len() > 0 { write!(f, " types: {:?}", self.types)?; }
        if self.func_addrs.len() > 0 { write!(f, " func_addrs: {:?}", self.func_addrs)?; }
        if self.table_addrs.len() > 0 { write!(f, " table_addrs: {:?}", self.table_addrs)?; }
        if self.mem_addrs.len() > 0 { write!(f, " mem_addrs: {:?}", self.mem_addrs)?; }
        if self.global_addrs.len() > 0 { write!(f, " global_addrs: {:?}", self.global_addrs)?; }
        write!(f, " }}")
    }
}

impl Debug for FuncInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncInst::Normal{ func_type: ft, module: _, code: Func(_, locals, Expr(instrs)) } => {
                write!(f, "NORMAL<type:{:?} locals:{:?} {:?}>", ft, locals, instrs)
            }
            FuncInst::Host{ func_type: ft, host_code: hc } => {
                write!(f, "HOST<type:{:?} {}>", ft, hc)
            },
        }
    }
}
