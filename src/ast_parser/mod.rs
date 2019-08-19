mod error;
mod module;
#[macro_use]mod util;

// mod type_parser;
// mod import_parser;
// mod table_parser;
// mod memory_parser;
// mod global_parser;
// mod func_parser;
// mod export_parser;
// mod start_parser;
// mod elem_parser;
// mod data_parser;

use annot::{Annot, Loc};
use context::*;
use lexer::*;
use cst_parser::*;

pub use self::error::*;
pub use self::module::*;

pub struct AstParser<'a> {
    lookahead: Option<&'a Cst>, 
    context: Context,
}

impl<'cst> AstParser<'cst> {

pub fn new(cst: &'cst Cst) -> Self {
    AstParser { lookahead: Some(cst), context: Context::default() }
}

pub fn parse(&mut self) -> Result<Module, AstParseError> {
    // self.parse_module()

    self.visit(self.lookahead.unwrap());

    Err(AstParseError::LastItem)
}

fn visit(&self, cst: &Cst) {
    match cst {
        Tree::Node(list) => {
            println!("{:?}", TokenKind::LeftParen);
            for elem in list {
                self.visit(elem);
            }
            println!("{:?}", TokenKind::RightParen);
        },
        Tree::Leaf(token) => {
            p!(token);
        },
    }    
}

fn parse_module(&mut self) -> Result<Module, AstParseError> {
    let mut module = Module::default();    

    self.match_list()?;
    self.match_list()?;
    self.match_keyword(Keyword::Module)?;

    // sort fields
    // v.sort_by_key(|k| {
    //     println!("{:?}", k);
    //     // match k.unwrap_node()[0] {
    //     //     tk!(TokenKind::Type) => 1,
    //     //     tk!(TokenKind::Import) => 2,
    //     //     tk!(TokenKind::Table) => 3,
    //     //     tk!(TokenKind::Memory) => 4,
    //     //     tk!(TokenKind::Global) => 5,
    //     //     tk!(TokenKind::Func) => 6,
    //     //     tk!(TokenKind::Export) => 7,
    //     //     tk!(TokenKind::Start) => 8,
    //     //     tk!(TokenKind::Elem) => 9,
    //     //     tk!(TokenKind::Data) => 10,
    //     //     _ => 0,
    //     // }
    // });

    // // normal module
    // let mut v_iter = v.iter();

    // v_iter.next().ok_or(self.err_last(&self.lookahead))
    // .and_then(|cst| {
    //     self.match_keyword(cst, Keyword::Module)
    // })?;

    // // module id
    // let mut v_iter_peekable = v_iter.peekable();
    // module.id = make_optional_id!(v_iter_peekable);

    // // parse fields
    // for field in v_iter_peekable {
    //     println!("field: {:?}", field);
    //     let field_v = self.match_list(&field)?;
    //     let mut field_iter = field_v.iter();

    //     // match field_iter.next() {
    //     //     Some(kw!(Keyword::Type)) => self.parse_type(&mut module, &mut context, field)?,
    //     //     Some(kw!(Keyword::Import)) => { self.parse_import(&mut module); },
    //     //     Some(kw!(Keyword::Table)) => { self.parse_table(&mut module); },
    //     //     Some(kw!(Keyword::Memory)) => { self.parse_memory(&mut module); },
    //     //     Some(kw!(Keyword::Global)) => { self.parse_global(&mut module); },
    //     //     Some(kw!(Keyword::Func)) => { self.parse_func(&mut module); },
    //     //     Some(kw!(Keyword::Export)) => { self.parse_export(&mut module); },
    //     //     Some(kw!(Keyword::Start)) => { self.parse_start(&mut module); },
    //     //     Some(kw!(Keyword::Elem)) => { self.parse_elem(&mut module); },
    //     //     Some(kw!(Keyword::Data)) => { self.parse_data(&mut module); },
    //     //     _ => {},
    //     // }
    // }

    Ok(module)
}

// fn err_last(&self, prev_cst: &Cst) -> AstParseError {
//     AstParseError::LastItem(prev_cst.clone())
// }

// fn err(&self) -> AstParseError {
//     AstParseError::Invalid(self.lookahead.clone())
// }

fn match_list<'a>(&mut self) -> Result<(), AstParseError> {
    match self.lookahead {
        Some(Tree::Node(list)) => {
            let mut iter = list.iter();
            self.lookahead = iter.next();
            Ok(())
        },
        Some(cst) => Err(AstParseError::Invalid(cst.clone())),
        _ => Err(AstParseError::LastItem)
    }
}

fn match_keyword(&mut self, matching: Keyword) -> Result<(), AstParseError> {
    match self.lookahead {
        Some(kw!(kw)) => {
            if kw == &matching {
                // self.lookahead = 
                Ok(())
            } else {
                Err(AstParseError::Invalid(self.lookahead.unwrap().clone()))
            }
        },
        Some(cst) => Err(AstParseError::Invalid(cst.clone())),
         _ => Err(AstParseError::LastItem),
    }
}

// fn optional_id(&self)
}

