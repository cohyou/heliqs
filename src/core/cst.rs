use std::fmt::Debug;

use super::token::Token;
use core::token::ValType;

#[derive(PartialEq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}

macro_rules! expecting {
    ($s:ident, $p:pat, $r:expr, $m: expr) => {
        if let $p = $s {
            $r
        } else {
            panic!("{} {:?}", $m, $s);
        }        
    };
}

pub type CST = Tree<Token>;

impl CST {
    pub fn unwrap_node(&self) -> Vec<Tree<Token>> {
        if let Tree::Node(v) = self { v.to_vec() } else { panic!("あかーん"); }
    }

    pub fn expect_node(&self, message: &'static str) -> Vec<Tree<Token>> {
        if let Tree::Node(v) = self { v.to_vec() } else { panic!(message); }
    }

    pub fn expect_text(&self, message: &'static str) -> String {
        if let Tree::Leaf(Token::Text(s)) = self { s.clone() } else { panic!(message); }
    }

    pub fn expect_leaf(&self, message: &'static str) -> Token {
        if let Tree::Leaf(t) = self { t.clone() } else { panic!(message); }        
    }

    pub fn expect_symbol(&self, message: &'static str) -> String {
        expecting!(self, Tree::Leaf(Token::Symbol(s)), s.clone(), message)
    }

    pub fn expect_name(&self, message: &'static str) -> String {
        expecting!(self, Tree::Leaf(Token::Name(n)), n.clone(), message)
    }

    pub fn expect_valtype(&self, message: &'static str) -> ValType {
        expecting!(self, Tree::Leaf(Token::ValType(vt)), vt.clone(), message)
    }

    pub fn match_token(&self, token: Token) -> bool {
        if let Tree::Leaf(t) = self {
            t == &token
        } else {
            false
        }
    }
}

impl Debug for CST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Tree::Leaf(l) => {
                write!(f, "{:?}", l)
            },
            Tree::Node(v) => {
                write!(f, "{:?}", v)
            },
        }
    }
}
