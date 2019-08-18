use core::*;

macro_rules! err {
    ($mes:expr,$token:ident) => { ParseError::make($mes, $token.clone()) };
}

#[derive(Debug)]
pub enum ParseError {
    Invalid(String, Token),
    StrToNum(Token),
}

// pub type ParseError = Annot<ParseErrorKind>;

impl ParseError {
    
    pub fn make<'a>(message: &'a str, token: Token) -> Self {
        ParseError::Invalid(message.to_string(), token)
    }
}