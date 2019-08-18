use std::iter::repeat;

use instr::*;
use super::*;

// parseの途中部分を構成する場合があるので、この部分は関数化できない
macro_rules! parse_typeuse {
    ($v_iter_peekable:ident, $context:ident, $context_l:ident) => {

    $v_iter_peekable.next()
    .and_then(|cst| parse_typeuse_typeidx(cst, $context)
    .and_then(|idx| {        

        // params (0~)
        let mut num_of_params = 0;
        loop {
            if let Some(cst) = $v_iter_peekable.peek()
                .and_then(|cst| cst.is_node_with_token_type(TokenKind::Param)) {

                $v_iter_peekable.next();

                perse_param(cst)
                .and_then(|(id, vt)| {
                    let pidx = num_of_params;

                    let param_defs = &$context_l.typedefs[idx as usize].0;
                    $context_l.locals = repeat(None).take(param_defs.len()).collect();

                    let vt_def = &param_defs[pidx];
                    if vt == vt_def {
                        Some(id.map(|s| s.clone()))
                    } else {
                        None
                    }
                })
                .map(|id| {
                    let pidx = num_of_params;
                    $context_l.locals[pidx] = id.map(|s| s.clone())
                });

                num_of_params += 1;
            } else {
                break;
            }
        }
        if num_of_params != $context_l.locals.len() { return None; }

        // result (0 or 1)
        let mut num_of_results = 0;
        if let Some(cst) = $v_iter_peekable.peek()
            .and_then(|cst| cst.is_node_with_token_type(TokenKind::FuncResult)) {

            if let Some(vt) = parse_result(cst) {
                let vt_def = &$context_l.typedefs[idx as usize].1[0];                    
                if vt != vt_def { return None; }
            }
            num_of_results += 1;
        }
        let def_of_results = $context_l.typedefs[idx as usize].1.len();
        
        if num_of_results != def_of_results { return None; }

        Some(idx)
    })  // .and_then(|idx| {
    )  // .and_then(|cst| make_typeuse_typeidx(cst, $context)

    }
}

impl AstParser {

pub fn parse_type(&self, module: &mut Module, context: &mut Context, cst: &Cst) -> Result<(), AstParseError> {
    let mut tp = FuncType::default();
    let mut context_l = context.clone();

    let v = self.match_list(cst)?;
    let mut v_iter = v.iter();

    v_iter.next().ok_or(self.err())
    .and_then(|cst| self.match_keyword(cst, Keyword::Type) )?;

    // type id
    let mut v_iter_peekable = v_iter.peekable();
    context.types.push(make_optional_id!(v_iter_peekable));

    // typeuse
    parse_typeuse!(v_iter_peekable, context, context_l);

    module.types.push(tp);

    // Err(self.err())
    Ok(())
}

}

fn parse_typeuse_typeidx<'a>(cst: &Cst, context: &Context) -> Option<TypeIndex> {
    cst.list().and_then(|v| {
        let mut v_iter = v.iter();

        // 'type'
        v_iter.next();

        // typeidx
        v_iter.next()
        .and_then(|cst| cst.token())
        .and_then(|token| make_idx(token, &context.types).ok())
    })
}

fn perse_param(cst: &Cst) -> Option<(Option<&Id>, &ValType)> {
    cst.list().and_then(|v| {
        let mut v_iter = v.iter();

        // 'param'
        v_iter.next();

        // id
        let mut v_iter_peekable = v_iter.peekable();
        let id = make_optional_id!(v_iter_peekable);

        // ValType
        v_iter_peekable.next()
        .and_then(|cst| cst.valtype())
        .map(|vt| (id, vt))
    })
}

fn perse_result(cst: &Cst) -> Option<&ValType> {
    cst.list().and_then(|v| {
        let mut v_iter = v.iter();

        // 'result'
        v_iter.next();

        // ValType
        v_iter.next()
        .and_then(|token| token.valtype())
        .map(|vt| vt)
    })
}

fn perse_local(cst: &Cst) -> Option<(Option<&Id>, &ValType)> {
    cst.list().and_then(|v| {
        let mut v_iter = v.iter();

        // 'local'
        v_iter.next();

        // id
        let mut v_iter_peekable = v_iter.peekable();
        let id = make_optional_id!(v_iter_peekable);

        // ValType
        v_iter_peekable.next()
        .and_then(|cst| cst.valtype())
        .map(|vt| (id, vt))
    })
}
