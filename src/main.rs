mod core;
mod lexer;
mod parser;
mod module_maker;

use std::fs::File;

fn main() {
    let file_name = "a.wat";
    let mut file = File::open(file_name).unwrap();

    // while let Some(t) = lexer::lex(&mut file) {
    //     println!("{:?}", t);
    // }
    let mut p = parser::Parser::new();
    if let Some(cst) = p.parse(&mut file) {
        println!("CST:    {:?}", cst);
        println!("Module: {:?}", module_maker::make_module(cst));
    }
}
