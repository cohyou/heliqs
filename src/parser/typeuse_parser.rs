use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek { 
    pub(super) fn parse_typeuse(&mut self) -> Result<TypeIndex, ParseError> {
        self.match_lparen()?;
        let typeidx = self.parse_typeuse_typeidx()?;

        let mut tp = FuncType::default();

        self.parse_signature(&mut tp)?;

        self.check_typeuse(typeidx, tp)?;

        Ok(typeidx)
    }

    pub(super) fn parse_signature(&mut self, tp: &mut FuncType) -> Result<(), ParseError> {

        // params        
        loop {
            if self.is_lparen()? {
                self.match_lparen()?;
                if let kw!(Keyword::Param) = self.lookahead {
                    if let Ok(param_vt) = self.parse_param() {
                        tp.0.push(param_vt);
                    }
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
                self.match_rparen()?;
            },
            kw!(Keyword::Result) => {
                if let Ok(result_vt) = self.parse_result() {
                    tp.1.push(result_vt);
                }
            }
            _ => return Err(self.err())
        }

        Ok(())
    }

    fn check_typeuse(&mut self, typeidx: TypeIndex, tp: FuncType) -> Result<(), ParseError> {
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

        let res = self.resolve_id(&self.contexts[0].funcs.clone())?;

        self.match_rparen()?;

        Ok(res)
    }

    pub(super) fn parse_param(&mut self) -> Result<ValType, ParseError> {

        self.match_keyword(Keyword::Param)?;

        // param id
        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            if self.contexts.len() == 2 {
                let new_s = s.clone();
                self.context().types.push(Some(new_s));
            }
            self.consume()?;
        } else {
            if self.contexts.len() == 2 {
                self.context().types.push(None);
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

// parseの途中部分を構成する場合があるので、この部分は関数化できない
// macro_rules! parse_typeuse {
//     ($v_iter_peekable:ident, $context:ident, $context_l:ident) => {

//     $v_iter_peekable.next()
//     .and_then(|cst| parse_typeuse_typeidx(cst, $context)
//     .and_then(|idx| {

//         // params (0~)
//         let mut num_of_params = 0;
//         loop {
//             if let Some(cst) = $v_iter_peekable.peek()
//                 .and_then(|cst| cst.is_node_with_token_type(TokenKind::Param)) {

//                 $v_iter_peekable.next();

//                 perse_param(cst)
//                 .and_then(|(id, vt)| {
//                     let pidx = num_of_params;

//                     let param_defs = &$context_l.typedefs[idx as usize].0;
//                     $context_l.locals = repeat(None).take(param_defs.len()).collect();

//                     let vt_def = &param_defs[pidx];
//                     if vt == vt_def {
//                         Some(id.map(|s| s.clone()))
//                     } else {
//                         None
//                     }
//                 })
//                 .map(|id| {
//                     let pidx = num_of_params;
//                     $context_l.locals[pidx] = id.map(|s| s.clone())
//                 });

//                 num_of_params += 1;
//             } else {
//                 break;
//             }
//         }
//         if num_of_params != $context_l.locals.len() { return None; }

//         // result (0 or 1)
//         let mut num_of_results = 0;
//         if let Some(cst) = $v_iter_peekable.peek()
//             .and_then(|cst| cst.is_node_with_token_type(TokenKind::FuncResult)) {

//             if let Some(vt) = parse_result(cst) {
//                 let vt_def = &$context_l.typedefs[idx as usize].1[0];
//                 if vt != vt_def { return None; }
//             }
//             num_of_results += 1;
//         }
//         let def_of_results = $context_l.typedefs[idx as usize].1.len();

//         if num_of_results != def_of_results { return None; }

//         Some(idx)
//     })  // .and_then(|idx| {
//     )  // .and_then(|cst| make_typeuse_typeidx(cst, $context)

//     }
// }
