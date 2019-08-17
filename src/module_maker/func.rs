use std::str::FromStr;
use std::convert::TryFrom;
use std::iter::repeat;

use core::*;
use super::error::*;
use super::util::*;
use super::instr::*;


macro_rules! make_optional_id {
    ($peekable:ident) => {
        $peekable.peek()
        .and_then(|token| token.id())
        .map(|id| {
            $peekable.next();
            id
        })
    }
}

fn make_param(cst: &CST) -> Option<(Option<&Id>, &ValType)> {
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

fn make_result(cst: &CST) -> Option<&ValType> {
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

fn make_local(cst: &CST) -> Option<(Option<&Id>, &ValType)> {
    cst.list().and_then(|v| {
        let mut v_iter = v.iter();

        // 'local'
        v_iter.next();

        // id
        let mut v_iter_peekable = v_iter.peekable();
        let id = make_optional_id!(v_iter_peekable);
println!("id: {:?}", id);
        // ValType
        v_iter_peekable.next()
        .and_then(|cst| cst.valtype())
        .map(|vt| (id, vt))
    })
}

fn make_typeuse_typeidx<'a>(cst: &CST, context: &Context) -> Option<TypeIndex> {
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

macro_rules! make_typeuse {
    ($v_iter_peekable:ident, $context:ident, $context_l:ident) => {{
        let idx = $v_iter_peekable.next()
        .map(|cst| make_typeuse_typeidx(cst, $context)
        .map(|idx| {
            // { $before_params }

            // params (0~)
            let mut num_of_params = 0;
            loop {
                if let Some(cst) = $v_iter_peekable.peek()
                    .and_then(|cst| cst.is_node_with_token_type(TokenKind::Param)) {

                    $v_iter_peekable.next();

                    make_param(cst)
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

                make_result(cst)
                .map(|vt| {
                    let vt_def = &$context_l.typedefs[idx as usize].1[0];
                    // mystery...
                    if vt != vt_def { return None; } else { Some(()) }
                });

                num_of_results += 1;
            }
            let def_of_results = $context_l.typedefs[idx as usize].1.len();
            // mystery...
            if num_of_results != def_of_results { return None; } else { return Some(idx); }
        })
        ).unwrap();
        idx.unwrap().unwrap() }
    };
}

pub fn make_call_indirect<'a, Iter>(v: &mut Iter, context: &Context) -> Option<Instr>
    where Iter: Iterator<Item=&'a CST> {
    let mut v_iter_peekable = v.peekable();
    let mut context_l = context.clone();

    // typeuse
    let typeidx = make_typeuse!(v_iter_peekable, context, context_l);

    // 本来はcontext_lのlocalsが全てNoneなのを確かめるべきだがまたあとで

    Some(Instr::CallIndirect(0))  // どこかでうまく動かないので固定0を入れておく
}

pub fn make_func(cst: &CST, context: &mut Context) -> Option<Func> {
    let mut func = Func::new();
    let mut context_l = context.clone();

    cst.list().map(|v| {
        let mut v_iter = v.iter();

        // 'func'
        v_iter.next();

        // func id
        let mut v_iter_peekable = v_iter.peekable();
        let id = make_optional_id!(v_iter_peekable);

        // register to context
        context.funcs.push(id.map(|v| v.clone()));

        // typeuse
        func.func_type = make_typeuse!(v_iter_peekable, context, context_l);

        // locals (0~)
        loop {
            if let Some(cst) = v_iter_peekable.peek()
                .and_then(|cst| cst.is_node_with_token_type(TokenKind::Local)) {

                v_iter_peekable.next();

                make_local(cst)
                .map(|(id, vt)| {
                    func.locals.push(vt.clone());
                    context_l.locals.push(id.map(|s| s.clone()))
                });
            } else {
                break;
            }
        }

        // instrs (0~)
        make_instrs(&mut v_iter_peekable, &context_l)
        .map(|instrs| func.body.instrs = instrs );
    });

    Some(func)
}

#[test]
fn test_make_param_1() {
    // have id
    let v = vec![
        Tree::Leaf(Token::param(Loc(0, 0))),
        Tree::Leaf(Token::id("param1".to_string(), Loc(0, 0))),
        Tree::Leaf(Token::val_type(ValType::I32, Loc(0, 0))),
    ];
    let cst = Tree::Node(v);
    assert_eq!(make_param(&cst), Some( (Some(&"param1".to_string()), &ValType::I32) ));
}

#[test]
fn test_make_param_2() {
    // have id
    let v = vec![
        Tree::Leaf(Token::param(Loc(0, 0))),
        Tree::Leaf(Token::val_type(ValType::I32, Loc(0, 0))),
    ];
    let cst = Tree::Node(v);
    assert_eq!(make_param(&cst), Some( (None, &ValType::I32) ));
}

#[test]
fn test_make_result() {
    let v = vec![
        Tree::Leaf(Token::func_result(Loc(0, 0))),
        Tree::Leaf(Token::val_type(ValType::I64, Loc(0, 0))),
    ];
    let cst = Tree::Node(v);
    assert_eq!(make_result(&cst), Some(&ValType::I64));
}

#[test]
fn test_make_typeuse_typeidx_1() {
    let v = vec![
        Tree::Leaf(Token::func_type(Loc(0, 0))),
        Tree::Leaf(Token::symbol("42".to_string(), Loc(0, 0))),
    ];
    let cst = Tree::Node(v);

    let context = Context::new();
    assert_eq!(make_typeuse_typeidx(&cst, &context), Some(42));
}

#[test]
fn test_make_typeuse_typeidx_2() {
    let id = "testid";
    let v = vec![
        Tree::Leaf(Token::func_type(Loc(0, 0))),
        Tree::Leaf(Token::id(id.to_string(), Loc(0, 0))),
    ];
    let cst = Tree::Node(v);

    let mut context = Context::new();
    context.types.push(None);
    context.types.push(Some(id.to_string()));
    context.types.push(None);
    assert_eq!(make_typeuse_typeidx(&cst, &context), Some(1));
}

#[test]
fn test_make_func_1() {
    let v = vec![
        Tree::Leaf(Token::func(Loc(0, 0))),
        Tree::Leaf(Token::id("testfunc".to_string(), Loc(0, 0))),
        Tree::Node(vec![
            Tree::Leaf(Token::func_type(Loc(0, 0))),
            // Tree::Leaf(Token::symbol("1".to_string(), Loc(0, 0))),
            Tree::Leaf(Token::id("p1".to_string(), Loc(0, 0))),
        ]),
        Tree::Leaf(Token::nop(Loc(0, 0))),
    ];
    let cst = Tree::Node(v);

    let mut context = Context::new();
    context.types.push(Some("p0".to_string()));
    context.types.push(Some("p1".to_string()));
    context.typedefs.push( (vec![], vec![]) );
    context.typedefs.push( (vec![ValType::I32], vec![ValType::I64]) );

    assert_eq!(make_func2(&cst, &mut context), None);
}

// unused
// fn make_func(cst: &CST, context: &mut Context) -> Option<Func> {
//     // println!("make_func cst: {:?}", cst);
//     let mut func = Func::new();

//     let v = cst.unwrap_node();
//     let mut v_iter = v.iter().peekable();
//     // let mut v_iter = v.iter();
//     v_iter.next();  // cstの0番目はFuncだが、すでにチェック済みなので無視

//     // Funcのid
//     make_id!(v_iter, context.funcs);

//     let (mut local_context, next_, new_v_iter_) = make_typeuse(&mut v_iter, context)
//         .map(|(typeuse, local_context, next, new_v_iter)| {
//             func.func_type = typeuse;
//             (local_context, next, new_v_iter)
//     }).expect("local contextを取得できず");


//     // let mut next = next_;

//     // 戻ってきた項目が存在すれば、interatorの先頭に挿入し直す
//     // let next =
//     // if let Some(n) = next_ {
//     //     let mut new_vec: Vec<_> = v_iter.collect().to_vec();
//     //     new_vec.insert(0, n);
//     //     v_iter = new_vec.iter();
//     //     v_iter.next()
//     // } else {
//     //     next_
//     // };

//     let mut v_iter = new_v_iter_.peekable();
//     let mut next = next_;
//     println!("locals next: {:?}", next);
//     loop {
//         // println!("locals next: {:?}", next);
//         if let Some(Tree::Node(lv)) = next {
//             let mut lv_iter = lv.iter();
//             let local_lv_next = lv_iter.next();
//             if local_lv_next.is_none() { break; }  // 即不正というわけではない
//             if local_lv_next.unwrap().is_token_type(TokenKind::Local) {

//                 let mut local_vt_next = lv_iter.next();

//                 // idがくればそれをcontextに登録
//                 let mut local_pushed = false;  // 二重登録防止
//                 if let Some(tk!(TokenKind::Id(n))) = local_vt_next {
//                     local_context.locals.push(Some(n.clone()));
//                     local_pushed = true;
//                     local_vt_next = lv_iter.next();
//                 }

//                 // 最後に型をみる
//                 if let Some(tk!(TokenKind::ValType(vt))) = local_vt_next {
//                     func.locals.push(vt.clone());
//                     if !local_pushed { local_context.locals.push(None) }
//                 } else {
//                     panic!("'local'の後にValTypeがこない");
//                 }

//             } else {
//                 break;  // instrかもしれないので終了
//             }
//         } else {
//             break;  // instrかもしれないので終了
//         }

//         // 次を取る前にチェック
//         // 明らかにダサいが他に方法が浮かばない
//         if let Some(&Tree::Node(lv)) = v_iter.peek() {
//             if lv.len() >= 2 {
//                 if let tk!(TokenKind::Local) = lv[0] {
//                     next = v_iter.next();
//                     continue;
//                 }
//             }
//         }
//         break;
//     }

//     // isntrs
//     make_instrs(&mut v_iter, &local_context).map(|instrs| {
//         func.body.instrs = instrs;
//     });

//     Some(func)
// }

