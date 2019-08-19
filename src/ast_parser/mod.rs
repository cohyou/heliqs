mod error;
mod module;
#[macro_use]mod util;
mod ast_iterator;

mod type_parser;
mod import_parser;
// mod table_parser;
// mod memory_parser;
// mod global_parser;
// mod func_parser;
// mod export_parser;
// mod start_parser;
// mod elem_parser;
// mod data_parser;

use std::iter::Peekable;
use annot::{Annot, Loc};
use context::*;
use lexer::*;
use cst_parser::*;

pub use self::error::*;
pub use self::module::*;
pub use self::ast_iterator::*;

pub struct AstParser<'a> {
    lookahead: &'a Cst, contexts: Vec<Context>, eol: &'a Cst
}

impl<'a> AstParser<'a> {

pub fn new<Iter>(iter: &mut Iter, eol: &'a Cst) -> Self 
    where Iter: Iterator<Item=(&'a Cst, usize)> {                
    if let Some( (cst, idx) ) = iter.next() {
        if let Tree::Node(v) = cst {
            AstParser { lookahead: &v[0], contexts: vec![Context::default()], eol: eol }
        } else {
            panic!("");    
        }        
    } else {
        panic!("");
    }
}

pub fn parse<Iter>(&mut self, iter: &mut Iter) -> Result<Module, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    self.match_lparen(iter)?;
    self.parse_module(iter)
}

fn parse_module<Iter>(&mut self, iter: &mut Iter) -> Result<Module, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    let mut module = Module::default();
    
    self.match_keyword(iter, Keyword::Module)?;

    if let tk!(TokenKind::Id(s)) = self.lookahead {
        self.consume(iter)?;
        module.id = Some(s.clone());        
    }
    
    loop {
pp!(1, self.lookahead);
        if !self.is_lparen()? { break; }

        self.match_lparen(iter)?;    
        if let kw!(Keyword::Type) = self.lookahead {            
            self.parse_type(&mut module, iter)?;
pp!(2, self.lookahead);
        } else {
            break;
        }
    }
pp!(11, iter.next());
pp!(12, iter.next());
    loop {
pp!(3, self.lookahead);
        if self.is_rparen()? { break; }

        if let kw!(Keyword::Import) = self.lookahead {
            self.parse_import(&mut module, iter)?;
        } else {
            break;
        }
    }


    // self.parse_table(&mut module);
    // self.parse_memory(&mut module);
    // self.parse_global(&mut module);
    // self.parse_func(&mut module);
    // self.parse_export(&mut module);
    // self.parse_start(&mut module);
    // self.parse_elem(&mut module);
    // self.parse_data(&mut module);

    self.match_rparen(iter)?;

    Ok(module)
}

fn match_lparen<Iter>(&mut self, iter: &mut Iter) -> Result<(), AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    match self.lookahead {
        Tree::Node(_) => {
            self.consume(iter)?;
            Ok(())
        },
        _ => Err(self.err())
    }
}

fn match_keyword<Iter>(&mut self, iter: &mut Iter, matching: Keyword) -> Result<(), AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    match self.lookahead {
        kw!(kw) => {
            if kw == &matching {
                self.consume(iter)?;
                Ok(())
            } else {
                Err(self.err())
            }
        },
        _ => Err(self.err()),
    }
}

fn match_rparen<Iter>(&mut self, iter: &mut Iter) -> Result<(), AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    if self.lookahead == self.eol {
        self.consume(iter)?;
        Ok(())
    } else {
        Err(self.err())
    }
    
}

fn is_lparen(&mut self) -> Result<bool, AstParseError> {
    if let Tree::Node(_) = self.lookahead { Ok(true) } else { Ok(false) }
}

fn is_rparen(&mut self) -> Result<bool, AstParseError> {
    if let tk!(TokenKind::RightParen) = self.lookahead { Ok(true) } else { Ok(false) }
}

fn consume<Iter>(&mut self, iter: &mut Iter) -> Result<(), AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {        
    self.lookahead = self.next_cst(iter)?; pp!(cnsm, self.lookahead);
    Ok(())
}

fn next_cst<Iter>(&mut self, iter: &mut Iter) -> Result<&'a Cst, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    if let Some(cst) = iter.next() {
        if let (Tree::Node(v), idx) = cst {
            if v.len() == idx {
                // self.eol = Tree::Leaf(TokenKind::right_paren(self.lookahead.loc.clone()));
                Ok(self.eol)
            } else {
                Ok(&v[idx])
            }            
        } else {
            panic!("cst in iterator must be node, not leaf")
        }
    } else {
        Err(AstParseError::LastItem)
    }
}

fn context(&mut self) -> &mut Context {
    let len = self.contexts.len();
    &mut self.contexts[len - 1]
}

fn err(&self) -> AstParseError {
    AstParseError::Invalid(self.lookahead.clone())
}

// fn peek_cst<Iter>(&mut self, mut iter: Peekable<Iter>) -> Result<&'a Cst, AstParseError>
//     where Iter: Iterator<Item=(&'a Cst, usize)> {
//     if let Some(cst) = iter.peek() {
//         if let (Tree::Node(v), idx) = cst {
//             if v.len() == idx.clone() {
//                 Ok(self.eol)
//             } else {
//                 Ok(&v[idx.clone()])
//             }            
//         } else {
//             panic!("cst in iterator must be node, not leaf")
//         }
//     } else {
//         Err(AstParseError::LastItem)
//     }
// }

// fn visit(&self, cst: &Cst) {
//     match cst {
//         Tree::Node(list) => {
//             println!("{:?}", TokenKind::LeftParen);
//             for elem in list {
//                 self.visit(elem);
//             }
//             println!("{:?}", TokenKind::RightParen);
//         },
//         Tree::Leaf(token) => {
//             p!(token);
//         },
//     }
// }

}

