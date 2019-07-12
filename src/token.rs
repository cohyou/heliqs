#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Module,
    Import,
    Func,
    Symbol(String),
    Name(String), // $で始まる
    Empty,
}
