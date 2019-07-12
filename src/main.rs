mod lexer;
mod token;

use std::fs::File;

fn main() {
    let file_name = "a.wat";
    let mut file = File::open(file_name).unwrap();
    
    while let Some(t) = lexer::lex(&mut file) {
        println!("token: {:?}", t);
    }
}