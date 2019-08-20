#[macro_use]mod util;
mod error;
mod module;

mod type_parser;
mod import_parser;
mod typeuse_parser;

use std::io::{Read, Seek};
use std::convert::TryFrom;

use annot::*;
use context::*;
use instr::*;
use lexer::*;

pub use self::error::*;
pub use self::module::*;
pub use self::typeuse_parser::*;

pub struct Parser<R>
where R: Read + Seek {
    lexer: Lexer<R>,
    lookahead: Token,
    contexts: Vec<Context>,
}

impl<R> Parser<R> where R: Read + Seek {
    pub fn new(reader: R) -> Self {
        Self {
            lexer: Lexer::new(reader),
            lookahead: Token::empty(Loc::default()),
            contexts: vec![Context::default()],
        }
    }

    pub fn parse(&mut self) -> Result<Module, ParseError> {
        self.lookahead = self.lexer.next_token()?;
        self.match_lparen()?;
        self.parse_module()
    }

    fn err(&self) -> ParseError {
        ParseError::Invalid(self.lookahead.clone())
    }

    fn parse_module(&mut self) -> Result<Module, ParseError> {
        let mut module = Module::default();
        
        self.match_keyword(Keyword::Module)?;

        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            module.id = Some(s.clone());        
            self.consume()?;            
        }
        
        loop {
    
            if !self.is_lparen()? { break; }

            self.match_lparen()?;    
            if let kw!(Keyword::Type) = self.lookahead {            
                self.parse_type(&mut module)?;
    
            } else {
                break;
            }
        }
        loop {
    
            match self.lookahead {
                tk!(TokenKind::RightParen) => break,
                kw!(Keyword::Import) => {
                    self.parse_import(&mut module)?;
                    if self.is_lparen()? {
                        self.consume()?;
                    }
                },
                _ => return Err(self.err()),
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

        self.match_rparen()?;

        Ok(module)
    }

    fn match_keyword(&mut self, matching: Keyword) -> Result<(), ParseError> {
        match &self.lookahead {
            kw!(kw) => {
                if kw == &matching {
                    self.consume()?;
                    Ok(())
                } else {
                    Err(self.err())
                }
            },
            _ => Err(self.err()),
        }
    }

    fn is_lparen(&mut self) -> Result<bool, ParseError> {
        if let tk!(TokenKind::LeftParen) = self.lookahead { Ok(true) } else { Ok(false) }
    }

    fn is_rparen(&mut self) -> Result<bool, ParseError> {
        if let tk!(TokenKind::RightParen) = self.lookahead { Ok(true) } else { Ok(false) }
    }

    fn match_lparen(&mut self) -> Result<(), ParseError> {
        self.match_token(TokenKind::LeftParen)
    }

    fn match_rparen(&mut self) -> Result<(), ParseError> {
        self.match_token(TokenKind::RightParen)
    }

    fn match_token(&mut self, t: TokenKind) -> Result<(), ParseError> {
        if self.lookahead.value == t {
            self.consume()
        } else {
            Err(ParseError::NotMatch(self.lookahead.clone(), t))
        }
    }

    fn parse_name(&mut self) -> Result<Name, ParseError> {
        if let tk!(TokenKind::String(s)) = &self.lookahead {            
            let res = Ok(s.clone());
            self.consume()?;
            res
        } else {
            Err(ParseError::NotMatch(self.lookahead.clone(), TokenKind::String("".into())))
        }
    }

    fn parse_valtype(&mut self) -> Result<ValType, ParseError> {
        if let kw!(Keyword::ValType(vt)) = &self.lookahead {
            let res = vt.clone();
            self.consume()?;
            Ok(res)
        } else {
            Err(self.err())
        }
    }
    
    fn parse_num<T: TryFrom<usize>>(&mut self) -> Result<T, ParseError> {
        if let nm!(Number::Unsigned(n)) = &self.lookahead {            
            if let Ok(num) = T::try_from(n.clone()) {
                self.consume()?;
                Ok(num)
            } else {
                Err(ParseError::NumCast(self.lookahead.clone()))
            }
        } else {
            Err(self.err())
        }
    }

    fn resolve_id(&mut self, from: &Vec<Option<Id>>) -> Result<TypeIndex, ParseError> {
        match &self.lookahead {
            nm!(Number::Unsigned(n)) => {
                let res = TypeIndex::try_from(n.clone())?;
                self.consume()?;
                Ok(res)
            },
            tk!(TokenKind::Id(id)) => {
                
                if let Some(idx) = from.iter()
                // .inspect(|c| println!("before: {:?}", c))
                .position(|t|
                    if let Some(typeidx) = t {
                        typeidx == id
                    } else {
                        false
                    }
                ) {
                    self.consume()?;
                    Ok(TypeIndex::try_from(idx)?)
                } else {
                    Err(ParseError::CantResolveId(self.lookahead.clone()))
                }
            }
            _ => Err(self.err()),
        }
    }

    fn consume(&mut self) -> Result<(), ParseError> {
        self.lookahead = self.lexer.next_token()?;
        // p!(self.lookahead);
        Ok(())
    }

    fn context(&mut self) -> &mut Context {
        let len = self.contexts.len();
        &mut self.contexts[len - 1]
    }
}