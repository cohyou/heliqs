use std::fmt::Debug;
use annot::{Annot, Loc};
use super::keyword::*;

#[derive(PartialEq, Clone)]
pub enum Number {
    Unsigned(usize),
}

impl Debug for Number {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           Number::Unsigned(num) => write!(f, "{:?}", num),
        //    _ => write!(f, "{:?}", self)
       }        
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Empty,

    Keyword(Keyword),
    Number(Number),
    String(String),
    Id(String), // $で始まる
    LeftParen,
    RightParen,
    Reserved(String),
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn empty(loc: Loc) -> Self { Self::new(TokenKind::Empty, loc) }

    pub fn keyword(kw: Keyword, loc: Loc) -> Self { Self::new(TokenKind::Keyword(kw), loc) }
    pub fn number_u(num: usize, loc: Loc) -> Self { Self::new(TokenKind::Number(Number::Unsigned(num)), loc) }
    pub fn string(s: String, loc: Loc) -> Self { Self::new(TokenKind::String(s), loc) }
    pub fn id(n: String, loc: Loc) -> Self { Self::new(TokenKind::Id(n), loc) }
    pub fn left_paren(loc: Loc) -> Self { Self::new(TokenKind::LeftParen, loc) }
    pub fn right_paren(loc: Loc) -> Self { Self::new(TokenKind::RightParen, loc) }
    pub fn reserved(s: Vec<u8>, loc: Loc) -> Self { Self::new(TokenKind::Reserved(String::from_utf8(s).unwrap()), loc) }
}

impl Debug for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self.value {
           TokenKind::Keyword(kw) => write!(f, "{:?}<{:?}>", kw, self.loc),
           TokenKind::Number(num) => write!(f, "{:?}<{:?}>", num, self.loc),
           TokenKind::String(s) => write!(f, "{:?}<{:?}>", s, self.loc),
           TokenKind::Id(id) => write!(f, "${}<{:?}>", id, self.loc),
           TokenKind::Reserved(r) => write!(f, "Reserved({})<{:?}>", r, self.loc),
           _ => write!(f, "{:?}<{:?}>", self.value, self.loc)
       }        
    }
}