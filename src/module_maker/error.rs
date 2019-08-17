use core::*;

#[derive(Debug)]
pub enum ParseError {
    Invalid(String),
    StrToNum(Token),
}

impl ParseError {
    pub fn make<'a>(message: &'a str) -> Self {
        ParseError::Invalid(message.to_string())
    }
}