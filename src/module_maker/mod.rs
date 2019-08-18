#[macro_use]mod error;
#[macro_use]mod util;
mod instr;
mod func;

use std::convert::TryFrom;
use std::iter::repeat;
use core::{
    Annot, TokenKind, Token, Tree, CST, Module,
    Func, FuncType, Import, ImportDesc, Instr, Start,
    Memory, Context, TypeIndex, ValType, Limits,
    MemType, Table, TableType, ElemType, Global, GlobalType, Mutablity, Expr,
    Export, ExportDesc, FuncIndex, TableIndex, MemIndex, GlobalIndex,
    Elem, Data, MemArg, LocalIndex, Id, Loc,
};
pub use self::error::*;
pub use self::util::*;
pub use self::instr::*;
pub use self::func::*;


macro_rules! tk { ($kind:pat) => { Tree::Leaf(Annot{value: $kind, ..}) } }

macro_rules! slice_get {
    ($v:ident, $s:ident, $i:expr, $func_name:expr) => {
        let $v = $s.get($i).expect(&format!("{} {}.get({})", $func_name, stringify!($s), stringify!($i)));
    };
}

pub fn make_module(cst: CST) -> Option<Module> {
    let mut module = Module::new();
    let mut context = Context::new();

    let v = cst.expect_node("make_module cst is not node");

    let vv = v[0].expect_node("make_module v is not node");
    if vv.len() == 0 {
        panic!("make_module listが空っぽ");
    }

    if !vv[0].is_token_type(TokenKind::Module) {
        panic!("make_module Moduleトークンで始まっていない");
    }

    // 空のモジュール
    if vv.len() == 1 { return Some(module); }

    // 通常のモジュール
    let mut pos = 1;

    // module id なくてもいい
    if let tk!(TokenKind::Id(n)) = &vv[pos] {
        module.id = Some(n.clone());
        pos += 1;
    }

    // 書かれた順番で定義していくと具合が悪いので、まずソートする
    let mut sliced = vv[pos..].to_vec();
    // println!("sliced: {:?}", sliced);
    sliced.sort_by_key(|k| {
        match k.unwrap_node()[0] {
            tk!(TokenKind::Type) => 1,
            tk!(TokenKind::Import) => 2,
            tk!(TokenKind::Table) => 3,
            tk!(TokenKind::Memory) => 4,
            tk!(TokenKind::Global) => 5,
            tk!(TokenKind::Func) => 6,
            tk!(TokenKind::Export) => 7,
            tk!(TokenKind::Start) => 8,
            tk!(TokenKind::Elem) => 9,
            tk!(TokenKind::Data) => 10,
            _ => 0,
        }
    });

    for module_field in sliced {
        // println!("module_field: {:?}", module_field);
        let vvv = module_field.expect_node("module読込時エラー");

        match &vvv[0].expect_token_kind("module nodeの最初がkeywordではない") {
            TokenKind::Type => {
                let mut pos = 1;

                // type id
                let mut type_id = None;
                if let tk!(TokenKind::Id(n)) = &vvv[pos] {
                    // contextにnameを入れる、typedefsも入れる必要あり
                    type_id = Some(n.clone());
                    pos += 1;
                }

                make_functype(&vvv[pos]).map(|functype| {
                    context.typedefs.push(functype.clone());
                    module.types.push(functype);
                    context.types.push(type_id);
                });
            },

            TokenKind::Import => {
                let import = make_import(&module_field, &mut context).expect("import作れず");
                module.imports.push(import);
            },

            TokenKind::Func => {
                make_func(&module_field, &mut context).map(|func| {
                    module.funcs.push(func);
                });
            },

            TokenKind::Table => {
                make_table(&module_field, &mut context).map(|table| {
                    module.tables.push(table);
                });
            },

            TokenKind::Memory => {
                make_memory(&module_field, &mut context).map(|mem| {
                    module.mems.push(mem);
                });
            },

            TokenKind::Global => {
                make_global(&module_field, &mut context).map(|global| {
                    module.globals.push(global);
                });
            },

            TokenKind::Export => {
                let export = make_export(&module_field, &mut context).expect("export作れず");
                module.exports.push(export);
            },

            TokenKind::Start => { module.start = make_start(&module_field, &context); },

            TokenKind::Elem => {
                make_elem(&module_field, &mut context).map(|elem| {
                    module.elems.push(elem);
                });
            },

            TokenKind::Data => {
                make_data(&module_field, &mut context).map(|data| {
                    module.data.push(data);
                });
            },

            _ => {},
        }
    }

    return Some(module);
}

fn make_functype(cst: &CST) -> Option<FuncType> {
    // println!("make_functype cst: {:?}", cst);

    let mut param_types = vec![];
    let mut result_types = vec![];

    let v = cst.expect_node("make_functype typeの次がNodeじゃない");

    if !v[0].is_token_type(TokenKind::Func) {
        panic!("直下はFuncから始まる");
    }

    let mut result_scanned = false;
    for node in &v[1..] {
        let v = node.expect_node("make_functype Funcの次がNodeじゃない");
        match v[0] {
            tk!(TokenKind::Param) => {
                if result_scanned {
                    panic!("resultの後にparamは置けない");
                }

                // param id
                let mut pos = 1;
                if let tk!(TokenKind::Id(_)) = &v[pos] {
                    // functypeの定義でparamに名前をつけても文法エラーにはならないが、
                    // その名前は使うことがない
                    pos += 1;
                }

                if let tk!(TokenKind::ValType(vt)) = &v[pos] {
                    param_types.push(vt.clone());
                } else {
                    panic!("パラメータの型名が正しくない");
                }
            },
            tk!(TokenKind::FuncResult) => {
                result_scanned = true;

                if let tk!(TokenKind::ValType(vt)) = &v[1] {
                    result_types.push(vt.clone());
                } else {
                    panic!("戻り値の型名が正しくない");
                }
            },
            _ => {},
        }
    }

    Some((param_types, result_types))
}

// import ::= {module name, name name, desc importdesc}
// importdesc ::= func typeidx
//              | table tabletype
//              | mem memtype
//              | global globaltype

// まずは以下を
// (import "wasi" "print" (func (type 0)))
//
fn make_import(cst: &CST, context: &mut Context) -> Option<Import> {
    // println!("make_import cst: {:?}", cst);

    let v = cst.expect_node("make_import not node");

    slice_get!(v1, v, 1, "make_import");

    let module_name = v1.expect_text("make_import モジュール名が取れず");
    let element_name = v[2].expect_text("make_import 要素名が取れず");

    make_import_desc(&v[3], context).map(|imp_desc| {
        Import {
            module_name: module_name,
            element_name: element_name,
            desc: imp_desc,
        }
    })
}

macro_rules! make_id {
    ($iter:ident,$ctxt_v:expr) => {
        let next = $iter.peek();
        if let Some(tk!(TokenKind::Id(n))) = next {
            $ctxt_v.push(Some(n.clone()));
            $iter.next();
        }
    };
}

macro_rules! make_global_type {
    ($v:ident) => {{
        let mut global_type = GlobalType::default();

        match $v.next() {
            Some(tk!(TokenKind::ValType(vt))) => {
                // const
                global_type.0 = Mutablity::Const;
                global_type.1 = vt.clone();
            },
            Some(Tree::Node(v)) => {
                // var
                let mut v_iter = v.iter();
                if let Some(tk!(TokenKind::Mutable)) = v_iter.next() {
                    if let Some(tk!(TokenKind::ValType(vt))) = v_iter.next() {
                        global_type.0 = Mutablity::Var;
                        global_type.1 = vt.clone();
                    } else {
                        panic!("mutの後がValTypeじゃない");
                    }
                } else {
                    panic!("mutじゃない");
                }
            },
            _ => { panic!("globaltypeが作れない"); },
        }

        global_type
    }};
}

fn make_import_desc(cst: &CST, context: &mut Context) -> Option<ImportDesc> {
    let desc_node = cst.expect_node("make_import_desc 要素が取れない");
    let mut v_iter = desc_node.iter().peekable();

    let next = v_iter.next();

    match next {
        Some(tk!(TokenKind::Func)) => {
            // Funcのid
            make_id!(v_iter, context.funcs);

            make_typeuse(&mut v_iter, context).map(|(typeuse, _, _, _)| {
                ImportDesc::Func(typeuse)
            })
        },
        Some(tk!(TokenKind::Table)) => {
            // Tableのid
            make_id!(v_iter, context.tables);

            let (limits, next) = make_limits(&mut v_iter);
            // elemtypeはanyfuncなのでチェックする
            if let Some(tk!(TokenKind::AnyFunc)) = next {
                let table_type = TableType { limits: limits, elem_type: ElemType::AnyFunc };
                Some(ImportDesc::Table(table_type))
            } else {
                panic!("tableの最後がanyfuncじゃない");
            }
        },
        Some(tk!(TokenKind::Memory)) => {
            // Memoryのid
            make_id!(v_iter, context.mems);
            let mem_type = MemType { limits: make_limits(&mut v_iter).0 };
            Some(ImportDesc::Mem(mem_type))
        },
        Some(tk!(TokenKind::Global)) => {
            // Globalのid
            make_id!(v_iter, context.globals);
            let global_type = make_global_type!(v_iter);
            Some(ImportDesc::Global(global_type))
        },
        _ => None,
    }
}

fn make_typeuse<'a, Iter>(v_iter: &'a mut Iter, context: &Context)
    -> Option<(TypeIndex, Context, Option<&'a CST>, &'a mut Iter)>
    where Iter: Iterator<Item=&'a CST> {
    let mut v = v_iter.peekable();
    if let Some( (n_, new_context_, result_next_, v_) ) =
        v.next()
        .map(|typeidx| {
            let types = typeidx.expect_node("");
            let tk = types[1].expect_leaf("");
            make_typeidx(&tk, context)
        })
        .map(|n| {
            let nn = n.unwrap();
            (nn, &context.typedefs[nn as usize])
        })
        .map(|(n, def)| {

            // 仕様書にはっきりとは書かれていないが、localのcontextは元のcontextを複製して作ることにする
            // let mut new_context = Context::new();
            let mut new_context = context.clone();

            new_context.locals = repeat(None).take(def.0.len()).collect();

            let mut param_idx = 0;
            let mut val_types: Vec<ValType> = vec![];

            // paramを調べる
            let mut next;
            let mut result_next = None;
            loop {
                next = v.peek();
                if let Some(Tree::Node(vv)) = next {


                    let mut vv_iter = vv.iter();

                    if let Some(child_cst) = vv_iter.next() {
                        match child_cst.expect_token_kind("a") {
                            TokenKind::Param => {
                                let mut val_type: Option<ValType> = None;
                                vv_iter.next().map(|id| {
                                    match id.expect_token_kind("b") {
                                        TokenKind::Id(i) => {
                                            // idを入れる
                                            new_context.locals[param_idx] = Some(i.clone());
                                        },
                                        TokenKind::ValType(vt) => {
                                            val_type = Some(vt.clone());
                                        },
                                        _ => { panic!("paramの宣言が不正"); }
                                    }
                                });
                                param_idx += 1;

                                if val_type.is_none() {
                                    vv_iter.next().map(|elem| {
                                        if let TokenKind::ValType(vt) = elem.expect_token_kind("") {
                                            val_type = Some(vt.clone());
                                        } else {
                                            panic!("paramの宣言が不正");
                                        }
                                    });
                                }
                                val_types.push(val_type.unwrap());
                            },
                            _ => { break; }
                        }
                    }
                    result_next = Some(v.next());  // ここで初めて確定
                } else {
                    break;
                }
            }

            // resultを調べる
            // こちらは名前をつけられないので数が合っているかどうかだけ
            let mut num_of_result = 0;
            let mut result_type: Option<ValType> = None;
            // println!("next: {:?}", next);
            if let Some(Tree::Node(vv)) = next {
                let mut vv_iter = vv.iter();
                if let Some(child_cst) = vv_iter.next() {
                    if child_cst.expect_token_kind("") == &TokenKind::FuncResult {
                        num_of_result = 1;
                        result_type = vv_iter.next().map(|type_cst| {
                            if let TokenKind::ValType(vt) = type_cst.expect_token_kind("") {
                                vt.clone()
                            } else {
                                panic!("resultの型がとれなかった");
                            }
                        });
                    }
                }
            }

            let num_of_params = param_idx;

            // 両方ゼロの場合は無条件でチェックをスルー
            if num_of_params + num_of_result > 0 {
                // paramの宣言と使用がそれぞれ合っているか調べる
                // まずは数
                if def.0.len() != num_of_params {
                    panic!("宣言された型とparamの数が合わない");
                } else {
                    // 次に型
                    // println!("val_types: {:?}", val_types);
                    if def.0 != val_types {
                        panic!("宣言された型とparamの型が合わない");
                    }
                }

                // println!("result: {:?} {:?}", num_of_result, def.1.len());
                if num_of_result != def.1.len() {
                    panic!("宣言された型とresultの数が合わない");
                } else {
                    // 数が同じ場合は型をチェックする
                    if let Some(vt) = result_type {
                        let def1 = &def.1;
                        if vt != def1[0] {
                            panic!("宣言された型とresultの型が合わない");
                        }
                    }
                }
            }

            // typeuse終了(funcの中がlistでなくなった)
            // しかしこの基準はinstrの中にlistで始まるものが存在しないと仮定している
            // 本来ならそういったinstrがないかチェックすべき
            // println!("new_context: locals: {:?} globals: {:?}", new_context.locals, new_context.globals);
            // (n, new_context, next, v)
            // (n, new_context, None, v_iter)
            (n, new_context, result_next, &mut v)
        }) {
            let res = result_next_.and_then(|n| n);
return Some( (n_, new_context_, res, v_iter) );
        } else {
            return None;
        }
    // None
}

fn make_table(cst: &CST, context: &mut Context) -> Option<Table> {
    let mut table = Table::new();

    let v = cst.unwrap_node();
    let mut v_iter = v.iter().peekable();
    v_iter.next();  // cstの0番目はtableだが、すでにチェック済みなので無視

    // Tableのid
    make_id!(v_iter, context.tables);

    let (limits, next) = make_limits(&mut v_iter);
    table.table_type.limits = limits;

    // elemtypeはanyfuncなのでチェックする
    if let Some(tk!(TokenKind::AnyFunc)) = next {
        ;  //　OK
    } else {
        panic!("tableの最後がanyfuncじゃない");
    }

    Some(table)
}

fn make_memory(cst: &CST, context: &mut Context) -> Option<Memory> {
    let mut mem = Memory::new();

    let v = cst.unwrap_node();
    let mut v_iter = v.iter().peekable();
    v_iter.next();  // cstの0番目はMemoryだが、すでにチェック済みなので無視

    // Memoryのid
    make_id!(v_iter, context.mems);

    mem.memory_type.limits = make_limits(&mut v_iter).0;

    Some(mem)
}

fn make_limits<'a>(v: &mut (impl Iterator<Item=&'a CST>)) -> (Limits, Option<&'a CST>) {
    let mut limits = Limits { min: 0, max: None };

    // min
    if let Some(tk!(TokenKind::Symbol(s))) = v.next() {
        limits.min = s.parse::<u32>().expect("limits minがu32でない");
    } else {
        panic!("memory minが不正");
    }

    // max (optional)
    let mut next = v.next();
    if let Some(tk!(TokenKind::Symbol(s))) = next {
        let max = s.parse::<u32>().expect("limits minがu32でない");
        limits.max = Some(max);
        next = v.next();
    }

    (limits, next)
}

fn make_global(cst: &CST, context: &mut Context) -> Option<Global> {
    let mut global = Global::new();

    let v = cst.unwrap_node();
    let mut v_iter = v.iter().peekable();
    v_iter.next();  // cstの0番目はGlobalだが、すでにチェック済みなので無視

    // Globalのid
    make_id!(v_iter, context.globals);

    // GlobalType
    global.global_type = make_global_type!(v_iter);

    // Expr
    make_instrs(&mut v_iter, context).map(|instrs| {
        global.init = Expr { instrs: instrs };
    });

    // println!("context.globals: {:?}", context.globals);

    Some(global)
}

fn make_export(cst: &CST, context: &mut Context) -> Option<Export> {
    let v = cst.expect_node("not node");
    let mut v_iter = v.iter();
    v_iter.next();  // cstの0番目はexportだが、すでにチェック済みなので無視

    let mut element_name: String = "".to_string();
    if let Some(tk!(TokenKind::Text(t))) = v_iter.next() {
        // 要素名
        element_name = t.clone();
    } else {
        panic!("export 要素名が取れず");
    }

    make_export_desc(&v[2], context).map(|exp_desc| {
        Export {
            name: element_name,
            desc: exp_desc,
        }
    })
}

fn make_export_desc(cst: &CST, context: &mut Context) -> Option<ExportDesc> {
    let desc_node = cst.expect_node("make_export_desc 要素が取れない");
    let mut v_iter = desc_node.iter().peekable();

    let next = v_iter.next();

    match next {
        Some(tk!(TokenKind::Func)) => {
            if let Some(Tree::Leaf(t)) = v_iter.next() {
                let funcidx = make_funcidx(t, context);
                Some(ExportDesc::Func(funcidx.unwrap()))
            } else {
                panic!("export funcの次が不正");
            }
        },
        Some(tk!(TokenKind::Table)) => {
            if let Some(Tree::Leaf(t)) = v_iter.next() {
                let tableidx = make_tableidx(t, context);
                Some(ExportDesc::Table(tableidx.unwrap()))
            } else {
                panic!("export tableの次が不正");
            }
        },
        Some(tk!(TokenKind::Memory)) => {
            if let Some(Tree::Leaf(t)) = v_iter.next() {
                let memidx = make_memidx(t, context);
                Some(ExportDesc::Mem(memidx.unwrap()))
            } else {
                panic!("export memoryの次が不正");
            }
        },
        Some(tk!(TokenKind::Global)) => {
            if let Some(Tree::Leaf(t)) = v_iter.next() {
                let globalidx = make_globalidx(t, context);
                Some(ExportDesc::Global(globalidx.unwrap()))
            } else {
                panic!("export globalの次が不正");
            }
        },
        _ => None,
    }
}

// type Result<T> = std::result::Result<T, ParseError>;

fn make_typeidx(token: &Token, context: &Context) -> Result<TypeIndex, ParseError> {
    make_idx_func2!(token, TypeIndex, context.types)
    // make_idx_func!(token, TypeIndex, context.types)
}

fn make_funcidx(token: &Token, context: &Context) -> Result<FuncIndex, ParseError> {
    make_idx_func2!(token, FuncIndex, context.funcs)
}

fn make_tableidx(token: &Token, context: &Context) -> Result<TableIndex, ParseError> {
    make_idx_func2!(token, TableIndex, context.tables)
}

fn make_memidx(token: &Token, context: &Context) -> Result<MemIndex, ParseError> {
    make_idx_func2!(token, MemIndex, context.mems)
}

fn make_globalidx(token: &Token, context: &Context) -> Result<GlobalIndex, ParseError> {
    make_idx_func2!(token, GlobalIndex, context.globals)
}

fn make_start(cst: &CST, context: &Context) -> Option<Start> {
    // println!("make_start cst: {:?}", cst);
    let mut start = Start::default();

    let v = cst.expect_node("");
    let mut v_iter = v.iter();
    v_iter.next();  // cstの0番目はstartだが、すでにチェック済みなので無視

    if let Some(Tree::Leaf(t)) = v_iter.next() {
        let funcidx = make_funcidx(t, context);
        start.func = funcidx.unwrap();
    } else {
        panic!("startidxが不正");
    }

    Some(start)
}

fn make_elem(cst: &CST, context: &Context) -> Option<Elem> {
    let mut elem = Elem::default();

    let v = cst.expect_node("");
    let mut v_iter = v.iter();
    v_iter.next();  // cstの0番目はelemだが、すでにチェック済みなので無視

    // tableidx
    if let Some(Tree::Leaf(t)) = v_iter.next() {
        let tableidx = make_tableidx(t, context);
        elem.table = tableidx.unwrap();
    } else {
        panic!("elem tableidxが不正");
    }

    // (offset e:expr_i)
    if let Some(Tree::Node(offsets)) = v_iter.next() {
        let mut offsets_iter = offsets.iter();
        make_offset(&mut offsets_iter, context).map(|expr| {
            elem.offset = expr;
        });
    } else {
        panic!("offsetがリストじゃない");
    }

    // 残りはfuncidx
    for token in v_iter {
        println!("token: {:?}", token);
        if let Tree::Leaf(t) = token {
            let funcidx = make_funcidx(t, context);
            elem.init.push(funcidx.unwrap());
        } else {
            panic!("elem funcidxが不正");
        }
    }

    Some(elem)
}

fn make_data(cst: &CST, context: &Context) -> Option<Data> {
    let mut data = Data::default();

    let v = cst.expect_node("");
    let mut v_iter = v.iter();
    v_iter.next();  // cstの0番目はdataだが、すでにチェック済みなので無視

    // memidx
    if let Some(Tree::Leaf(t)) = v_iter.next() {
        let memidx = make_memidx(t, context);
        data.data = memidx.unwrap();
    } else {
        panic!("data memidxが不正");
    }

    // (offset e:expr_i)
    if let Some(Tree::Node(offsets)) = v_iter.next() {
        let mut offsets_iter = offsets.iter();
        make_offset(&mut offsets_iter, context).map(|expr| {
            data.offset = expr;
        });
    } else {
        panic!("offsetがリストじゃない");
    }

    if let Some(tk!(TokenKind::Text(t))) = v_iter.next() {
        data.init = t.clone();
    } else {
        panic!("data datastringが不正");
    }

    Some(data)
}

fn make_offset<'a>(v: &mut (impl Iterator<Item=&'a CST>), context: &Context) -> Option<Expr> {
    let mut expr = Expr::default();

    v.next();  // 最初はoffsetなので飛ばす

    make_instrs(v, context).map(|instrs| {
        expr.instrs = instrs;
    });

    Some(expr)
}

