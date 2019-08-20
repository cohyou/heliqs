use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_typeuse(&mut self, params: &mut Vec<ValType>, results: &mut Vec<ValType>) -> Result<TypeIndex, ParseError> {
        self.match_lparen()?;
        let typeidx = self.parse_typeuse_typeidx()?;
        lla!(1, self);
        if !self.is_rparen()? {
            self.parse_signature(params, results)?;
        }
        Ok(typeidx)
    }

    pub(super) fn parse_signature(&mut self, params: &mut Vec<ValType>, results: &mut Vec<ValType>) -> Result<(), ParseError> {

        // params
        loop {
            if self.is_lparen()? {
                // let peeked = self.lexer.next_token()?;
                // p!(peeked);
                self.match_lparen()?;
                if let kw!(Keyword::Param) = self.lookahead {
                    
                    // self.lookahead = peeked;
                    lla!(4, self);
                    if let Ok(param_vt) = self.parse_param() {
                        params.push(param_vt);
                    }
                    lla!(5, self);
                    // self.match_lparen()?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // result

        match self.lookahead {
            tk!(TokenKind::RightParen) => {
                // self.match_rparen()?;
            },
            kw!(Keyword::Result) => {
                // self.match_keyword(Keyword::Result)?;
                if let Ok(result_vt) = self.parse_result() {
                    results.push(result_vt);
                }
                if self.is_lparen()? {
                    self.match_lparen()?;
                }
            }            
            kw!(Keyword::Local) => {},
            _ => return Err(self.err2("can not parse result"))
        }

        Ok(())
    }

    pub(super) fn check_typeuse(&mut self, typeidx: TypeIndex, tp: FuncType) -> Result<(), ParseError> {
        let typedef = &self.contexts[0].typedefs[typeidx as usize];
        if tp.0.len() == 0 && tp.1.len() == 0 { return Ok(()) }
        if typedef != &tp {
            Err(ParseError::InvalidTypeuseDef(self.lookahead.clone(), typedef.clone(), tp))
        } else {
            Ok(())
        }
    }

    fn parse_typeuse_typeidx(&mut self) -> Result<TypeIndex, ParseError> {
        self.match_keyword(Keyword::Type)?;

        let res = self.resolve_id(&self.contexts[0].types.clone())?;

        self.match_rparen()?;

        Ok(res)
    }

    pub(super) fn parse_param(&mut self) -> Result<ValType, ParseError> {
lla!(101, self);
        self.match_keyword(Keyword::Param)?;
lla!(102, self);
        // param id
        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            if self.contexts.len() == 2 {
                let new_s = s.clone();
                self.context().locals.push(Some(new_s));
            }
            self.consume()?;
        } else {
            if self.contexts.len() == 2 {
                self.context().locals.push(None);
            }
        }

        // valtype
        let vt = self.parse_valtype()?;

        self.match_rparen()?;

        Ok(vt)
    }

    pub(super) fn parse_result(&mut self) -> Result<ValType, ParseError> {

        self.match_keyword(Keyword::Result)?;

        // valtype
        let vt = self.parse_valtype()?;

        self.match_rparen()?;

        Ok(vt)
    }
}