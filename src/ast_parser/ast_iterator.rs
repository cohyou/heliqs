use cst_parser::*;

enum CstState<'a> {
    Init,
    Pop,
    Push(&'a Cst),
    Next
}

pub struct AstIterator<'a> {
    init: &'a Cst, lookahead: Vec<(&'a Cst, usize)>,
}

impl<'a> AstIterator<'a> {
    pub fn new(cst: &'a Cst) -> Self {
        AstIterator { init: cst, lookahead: vec![] }
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

}

impl<'a> Iterator for AstIterator<'a> {
    type Item = (&'a Cst, usize);

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
        self.lookahead.last().map(|item| item.clone())
    }
}