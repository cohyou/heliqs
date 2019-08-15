use std::fmt::Debug;

use super::token::Token;
use core::Annot;
use core::token::{ValType, TokenKind};

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
        if let Tree::Leaf(Annot{ value: TokenKind::Text(s), ..}) = self { s.clone() } else { panic!(message); }
    }

    pub fn expect_leaf(&self, message: &'static str) -> TokenKind {
        if let Tree::Leaf(Annot{value: t, ..}) = self { t.clone() } else { panic!(message); }        
    }

    pub fn expect_symbol(&self, message: &'static str) -> String {
        expecting!(self, Tree::Leaf(Annot{ value: TokenKind::Symbol(s), ..}), s.clone(), message)
    }

    pub fn expect_name(&self, message: &'static str) -> String {
        expecting!(self, Tree::Leaf(Annot{ value: TokenKind::Id(n), ..}), n.clone(), message)
    }

    pub fn expect_valtype(&self, message: &'static str) -> ValType {
        expecting!(self, Tree::Leaf(Annot{ value: TokenKind::ValType(vt), ..}), vt.clone(), message)
    }

    pub fn is_token_type(&self, token_kind: TokenKind) -> bool {
        if let Tree::Leaf(Annot{value: t, ..}) = self {
            t == &token_kind
        } else {
            false
        }
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
