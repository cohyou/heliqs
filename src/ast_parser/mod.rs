mod error;
mod module;
#[macro_use]mod util;

mod type_parser;
mod import_parser;
mod table_parser;
mod memory_parser;
mod global_parser;
mod func_parser;
mod export_parser;
mod start_parser;
mod elem_parser;
mod data_parser;

use annot::{Annot, Loc};
use context::*;
use lexer::*;
use cst_parser::*;

pub use self::error::*;
pub use self::module::*;

pub struct AstParser { lookahead: Cst, }

impl AstParser {

pub fn new(cst: Cst) -> Self {
    AstParser { lookahead: cst }
}

pub fn parse(&mut self) -> Result<Module, AstParseError> {
    self.parse_module()        
    // Err(AstParseError::Invalid(Token::empty(Loc::default())))
}

fn parse_module(&mut self) -> Result<Module, AstParseError> {
    let mut module = Module::default();
    let mut context = Context::default();

    let root = self.match_list(&self.lookahead)?;
    let v = self.match_list(&root[0])?;

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

    // normal module
    let mut v_iter = v.iter();

    v_iter.next().ok_or(self.err_last(&self.lookahead))
    .and_then(|cst| {
        self.match_keyword(cst, Keyword::Module)
    })?;

    // module id
    let mut v_iter_peekable = v_iter.peekable();
    module.id = make_optional_id!(v_iter_peekable);

    // parse fields
    for field in v_iter_peekable {
        println!("field: {:?}", field);
        let field_v = self.match_list(&field)?;
        let mut field_iter = field_v.iter();

        match field_iter.next() {
            Some(kw!(Keyword::Type)) => self.parse_type(&mut module, &mut context, field)?,
            Some(kw!(Keyword::Import)) => { self.parse_import(&mut module); },
            Some(kw!(Keyword::Table)) => { self.parse_table(&mut module); },
            Some(kw!(Keyword::Memory)) => { self.parse_memory(&mut module); },
            Some(kw!(Keyword::Global)) => { self.parse_global(&mut module); },
            Some(kw!(Keyword::Func)) => { self.parse_func(&mut module); },
            Some(kw!(Keyword::Export)) => { self.parse_export(&mut module); },
            Some(kw!(Keyword::Start)) => { self.parse_start(&mut module); },
            Some(kw!(Keyword::Elem)) => { self.parse_elem(&mut module); },
            Some(kw!(Keyword::Data)) => { self.parse_data(&mut module); },
            _ => {},
        }
    }

    Ok(module)
}

fn err_last(&self, prev_cst: &Cst) -> AstParseError {
    AstParseError::LastItem(prev_cst.clone())
}

fn err(&self) -> AstParseError {
    AstParseError::Invalid(self.lookahead.clone())
}

fn match_list<'a>(&self, cst: &'a Cst) -> Result<&'a Vec<Cst>, AstParseError> {
    if let Tree::Node(list) = cst { Ok(list) } else {
         Err(AstParseError::Invalid(cst.clone()))
    }
}

fn match_keyword(&self, cst: &Cst, matching: Keyword) -> Result<Keyword, AstParseError> {
    if let tk!(TokenKind::Keyword(kw)) = cst {
        if kw == &matching { Ok(kw.clone()) } else {
            Err(AstParseError::Invalid(cst.clone()))            
        }
    } else {
        Err(AstParseError::Invalid(cst.clone()))
    }
}

}

