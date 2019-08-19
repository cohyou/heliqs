use std::fmt::Debug;

use annot::*;
use lexer::*;
use super::*;

macro_rules! tk { ($kind:pat) => { Tree::Leaf(Annot{value: $kind, ..}) } }

impl Cst {
    // pub fn list(&self) -> Option<&Vec<Tree<Token>>> {
    //     if let Tree::Node(list) = self { Some(list) } else { None }
    // }
    // pub fn keyword(&self, matching: Keyword) -> Option<Keyword> {
    //     if let tk!(TokenKind::Keyword(kw)) = self {
    //         if kw == &matching { Some(kw.clone()) } else { None }
    //     } else {
    //         None
    //     }
    // }

    pub fn id(&self) -> Option<&String> {
        if let tk!(TokenKind::Id(id)) = self { Some(id) } else { None }
    }

    // pub next(&mut self) -> Option<>
}

impl Debug for Cst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Tree::Leaf(l) => { write!(f, "{:?}", l) },
            Tree::Node(v) => { write!(f, "{:?}", v) },
        }
    }
}