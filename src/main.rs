mod core;
mod token;
mod lexer;
mod parser;

use std::fs::File;

fn main() {
    let file_name = "a.wat";
    let mut file = File::open(file_name).unwrap();

    // while let Some(t) = lexer::lex(&mut file) {
    //     println!("{:?}", t);
    // }
    let mut p = parser::Parser::new();
    p.parse_module(&mut file);
    println!("{:?}", p);
}
