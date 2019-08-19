// use core::*;

// macro_rules! err {
//     ($mes:expr,$token:ident) => { ParseError::make($mes, $token.clone()) };
// }

use cst_parser::Cst;

#[derive(Debug)]
pub enum AstParseError {
    Invalid(Cst),
    StrToNum(Cst),
    LastItem,
}

// use lexer::LexError;
// impl From<LexError> for CstParseError {
//     fn from(e: LexError) -> Self { CstParseError::Lex(e) }
// }

// pub type ParseError = Annot<ParseErrorKind>;

// impl ParseError {
    
//     pub fn make<'a>(message: &'a str, token: Token) -> Self {
//         ParseError::Invalid(message.to_string(), token)
//     }
// }