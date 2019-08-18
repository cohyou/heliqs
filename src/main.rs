extern crate heliqs;

use std::env;
use std::fs::File;
use heliqs::{CstParser, AstParser};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_name = &args[1];
    let mut reader = File::open(file_name).unwrap();

    let mut cst_parser = CstParser::new(&mut reader);
    match cst_parser.parse(&mut reader) {
        Err(err) => println!("CST PARSE ERROR: {:?}", err),
        Ok(cst) => {
            println!("CST: {:?}", cst);

            let mut ast_parser = AstParser::new(cst);
            match ast_parser.parse() {
                Err(err) => println!("AST PARSE ERROR: {:?}", err),
                Ok(module) => println!("MODULE: {:?}", module),
            }
        },
    }

    // let mut store = Runtime::init_store();
    // let func_inst = FuncInst::Host { func_type: (vec![ValType::I32], vec![]), host_code: "log".to_string() };
    // store.insts.push(StoreInst::Func(func_inst));

    // let mut rt = Runtime::new(Some(store));
    // let extern_vals = vec![ExternVal::Func(0)];
    // println!("module instance: {:?}", rt.instantiate(&module, extern_vals));
}