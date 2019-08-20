#[macro_use]extern crate heliqs;

use std::env;
use std::fs::File;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_name = &args[1];
    let mut reader = File::open(file_name).unwrap();

    // use heliqs::Lexer;
    // use heliqs::TokenKind;
    // let mut lexer = Lexer::new(&mut reader);
    // while let Ok(t) = lexer.next_token() {
    //     if t.value == TokenKind::Empty {
    //         break;
    //     }
    //     p!(t);
    // }

    use heliqs::Parser;
    let mut parser = Parser::new(reader);
    match parser.parse() {
        Ok(module) => println!("MODULE: {:?}", module),
        Err(err) => println!("PARSE ERROR: {:?}", err),
    }

    
    // use heliqs::{CstParser, AstParser};
    // let mut cst_parser = CstParser::new(&mut reader);
    // match cst_parser.parse(&mut reader) {
    //     Err(err) => println!("CST PARSE ERROR: {:?}", err),
    //     Ok(cst) => {
    //         // println!("CST: {:?}", cst);

    //         use heliqs::AstIterator;
    //         use heliqs::{Tree, Token, Loc};
    //         let mut iter = AstIterator::new(&cst);
            
    //         let rp = Tree::Leaf(Token::right_paren(Loc::default()));

    //         let mut ast_parser = AstParser::new(&mut iter, &rp);
    //         match ast_parser.parse(&mut iter) {
    //             Err(err) => println!("AST PARSE ERROR: {:?}", err),
    //             Ok(module) => println!("MODULE: {:?}", module),
    //         }
    //     },
    // }

    // let mut store = Runtime::init_store();
    // let func_inst = FuncInst::Host { func_type: (vec![ValType::I32], vec![]), host_code: "log".to_string() };
    // store.insts.push(StoreInst::Func(func_inst));

    // let mut rt = Runtime::new(Some(store));
    // let extern_vals = vec![ExternVal::Func(0)];
    // println!("module instance: {:?}", rt.instantiate(&module, extern_vals));
}