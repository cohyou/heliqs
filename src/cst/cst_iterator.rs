use annot::*;
use lexer::*;
use cst_parser::*;

enum CstState<'a> {
    Init,
    Pop,
    Push(&'a Cst),
    Next
}

pub struct CstIterator<'a> {
    init: &'a Cst, lookahead: Vec<(&'a Cst, usize)>,
}

impl<'a> CstIterator<'a> {
    pub fn new(cst: &'a Cst) -> Self {
        CstIterator { init: cst, lookahead: vec![] }
    }

fn cst_state(&self) -> Option<CstState<'a>> {
    if self.lookahead.len() == 0 {
        return Some(CstState::Init);
    }
    self.lookahead.last().map(|(cst, idx)| {
        match cst {
            Tree::Node(v) => {
                if &v.len() == idx {
                    CstState::Pop
                } else {
                    match &v[idx.clone()] {
                        n @ Tree::Node(_) => {
                            CstState::Push(n)
                        },
                        _ => CstState::Next,
                    }
                }
            },
            _ => panic!("cst_state"),
        }
    })
}

fn nearest_prev_leaf_loc(&self, item: &(&'a Cst, usize)) -> &'a Loc {
    &Loc(1, 0)
}

}

impl<'a> Iterator for CstIterator<'a> {
    type Item = &'a Cst;

    fn next(&mut self) -> Option<Self::Item> {
        self.cst_state().map(|state| {
            match state {
                CstState::Init => {
                    self.lookahead.push( (self.init, 0) );
                },
                CstState::Pop => {
                    self.lookahead.pop();
                    if let Some(last) = self.lookahead.pop() {
                        self.lookahead.push( (last.0, last.1 + 1) );
                    }
                },
                CstState::Push(child) => {
                    self.lookahead.push( (child, 0) );
                },
                CstState::Next => {
                    if let Some(last) = self.lookahead.pop() {
                        self.lookahead.push( (last.0, last.1 + 1) );
                    }
                },
            }
        });
        self.lookahead.last().map(|item| {
            pp!(next, item);

            if let (Tree::Node(v), idx) = item {
                if v.len() == idx.clone() {
                    let loc = self.nearest_prev_leaf_loc(item).added(1);
                    &Tree::Leaf(Token::right_paren(loc))
                } else {
                    &v[idx.clone()]
                }
            } else {
                panic!("cst in iterator must be node, not leaf")
            }
        })
    }
}