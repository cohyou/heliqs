mod lexer;
mod token;
mod wannaiter;

use std::fs::File;

fn main() {
    let file_name = "a.wat";
    let file = File::open(file_name).unwrap();

    let mut lexer = lexer::Lexer::new(file);
    while let Some(t) = lexer.next() {
        println!("token: {:?}", t);
    }
}