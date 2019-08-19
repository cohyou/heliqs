use std::iter::repeat;

use instr::*;
use super::*;

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

impl<'a> AstParser<'a> {

pub fn parse_type<Iter>(&mut self, module: &mut Module, iter: &mut Iter) -> Result<(), AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {

    self.match_keyword(iter, Keyword::Type)?;

    // type id
    if let tk!(TokenKind::Id(s)) = self.lookahead {
        self.consume(iter)?;
        self.context().funcs.push(Some(s.clone()));
    } else {
        self.context().funcs.push(None);
    }

    // functype    
    self.match_lparen(iter)?;
    let tp = self.parse_functype(iter)?;

    module.types.push(tp);
pp!(before_type_last, self.lookahead);
    self.match_rparen(iter)?;
pp!(after_type_last, self.lookahead);
    Ok(())
}

pub fn parse_functype<Iter>(&mut self, iter: &mut Iter) -> Result<FuncType, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    let mut tp = FuncType::default();

    self.match_keyword(iter, Keyword::Func)?;
    
    // params
    loop {
        if self.is_lparen()? {
            self.match_lparen(iter)?;
            if let kw!(Keyword::Param) = self.lookahead {
                if let Ok(param_vt) = self.parse_param(iter) {
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
            self.match_rparen(iter)?;
        },
        kw!(Keyword::Result) => {
            if let Ok(result_vt) = self.parse_result(iter) {
                tp.1.push(result_vt);
            }                        
        }
        _ => return Err(self.err())
    }

    Ok(tp)
}

pub fn parse_param<Iter>(&mut self, iter: &mut Iter) -> Result<ValType, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {

    self.match_keyword(iter, Keyword::Param)?;

    // param id
    if let tk!(TokenKind::Id(s)) = self.lookahead {
        self.consume(iter)?;
        if self.contexts.len() == 2 {
            self.context().types.push(Some(s.clone()));
        }
    } else {
        if self.contexts.len() == 2 {
            self.context().types.push(None);
        }
    }

    // valtype
    let vt = self.parse_valtype(iter)?;

    self.match_rparen(iter)?;

    Ok(vt)
}

pub fn parse_valtype<Iter>(&mut self, iter: &mut Iter) -> Result<ValType, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {
    if let kw!(Keyword::ValType(vt)) = self.lookahead {
        self.consume(iter)?;
        Ok(vt.clone())
    } else {
        Err(self.err())
    }
}

pub fn parse_result<Iter>(&mut self, iter: &mut Iter) -> Result<ValType, AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {

    self.match_keyword(iter, Keyword::Result)?;

    // valtype
    let vt = self.parse_valtype(iter)?;

    self.match_rparen(iter)?;

    Ok(vt)
}

}

// fn parse_typeuse_typeidx<'a>(cst: &Cst, context: &Context) -> Option<TypeIndex> {
//     cst.list().and_then(|v| {
//         let mut v_iter = v.iter();

//         // 'type'
//         v_iter.next();

//         // typeidx
//         v_iter.next()
//         .and_then(|cst| cst.token())
//         .and_then(|token| make_idx(token, &context.types).ok())
//     })
// }


// fn perse_local(cst: &Cst) -> Option<(Option<&Id>, &ValType)> {
//     cst.list().and_then(|v| {
//         let mut v_iter = v.iter();

//         // 'local'
//         v_iter.next();

//         // id
//         let mut v_iter_peekable = v_iter.peekable();
//         let id = make_optional_id!(v_iter_peekable);

//         // ValType
//         v_iter_peekable.next()
//         .and_then(|cst| cst.valtype())
//         .map(|vt| (id, vt))
//     })
// }
