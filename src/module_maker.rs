use std::iter::repeat;
use core::{
    Annot, TokenKind, Token, Tree, CST, Module,
    Func, FuncType, Import, ImportDesc, Instr, Start,
    Val, Memory, Context, TypeIndex, ValType, Limits,
    MemType, Table, TableType, ElemType, Global, GlobalType, Mutablity, Expr,
    Export, ExportDesc, FuncIndex, TableIndex, MemIndex, GlobalIndex,
    Elem, Data
};

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

    for module_field in &vv[pos..] {
        // println!("module_field: {:?}", module_field);
        let vvv = module_field.expect_node("module読込時エラー");

        match &vvv[0].expect_leaf("module nodeの最初がkeywordではない") {
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

            make_typeuse(&mut v_iter, context).map(|(typeuse, _, _)| {
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

fn make_func(cst: &CST, context: &mut Context) -> Option<Func> {
    // println!("make_func cst: {:?}", cst);
    let mut func = Func::new();

    let v = cst.unwrap_node();
    let mut v_iter = v.iter().peekable();
    v_iter.next();  // cstの0番目はFuncだが、すでにチェック済みなので無視

    // Funcのid
    make_id!(v_iter, context.funcs);

    let (mut local_context, next_) = make_typeuse(&mut v_iter, context)
        .map(|(typeuse, local_context, next)| {
            func.func_type = typeuse;
            (local_context, next)
    }).expect("local contextを取得できず");

    let mut next = next_;
    loop {
        // println!("locals next: {:?}", next);
        if let Some(Tree::Node(lv)) = next {
            let mut lv_iter = lv.iter();
            let local_lv_next = lv_iter.next();
            if local_lv_next.is_none() { break; }  // 即不正というわけではない
            if local_lv_next.unwrap().is_token_type(TokenKind::Local) {

                let mut local_vt_next = lv_iter.next();

                // idがくればそれをcontextに登録
                let mut local_pushed = false;  // 二重登録防止
                if let Some(tk!(TokenKind::Id(n))) = local_vt_next {
                    local_context.locals.push(Some(n.clone()));
                    local_pushed = true;
                    local_vt_next = lv_iter.next();
                }

                // 最後に型をみる
                if let Some(tk!(TokenKind::ValType(vt))) = local_vt_next {
                    func.locals.push(vt.clone());
                    if !local_pushed { local_context.locals.push(None) }
                } else {
                    panic!("'local'の後にValTypeがこない");
                }

            } else {
                break;  // instrかもしれないので終了
            }
        } else {
            break;
        }

        // 次を取る前にチェック
        // 明らかにダサいが他に方法が浮かばない
        if let Some(&Tree::Node(lv)) = v_iter.peek() {
            if lv.len() >= 2 {
                if let tk!(TokenKind::Local) = lv[0] {
                    next = v_iter.next();
                    continue;
                }
            }
        }
        break;
    }

    // isntrs
    make_instrs(&mut v_iter, &local_context).map(|instrs| {
        func.body.instrs = instrs;
    });

    Some(func)
}

fn make_typeuse<'a>(v: &mut (impl Iterator<Item=&'a CST>), context: &Context)
    -> Option<(TypeIndex, Context, Option<&'a CST>)> {
    v.next()
        .map(|typeidx| {
            let types = typeidx.expect_node("");
            let tk = types[1].expect_leaf("");
            make_typeidx(tk, context)
        })
        .map(|n| (n, &context.typedefs[n as usize]))
        .map(|(n, def)| {
            let mut new_context = Context::new();
            new_context.locals = repeat(None).take(def.0.len()).collect();

            let mut param_idx = 0;
            let mut val_types: Vec<ValType> = vec![];

            // paramを調べる
            let mut next;
            loop {
                next = v.next();
                if let Some(Tree::Node(vv)) = next {
                    let mut vv_iter = vv.iter();

                    if let Some(child_cst) = vv_iter.next() {
                        match child_cst.expect_leaf("a") {
                            TokenKind::Param => {
                                let mut val_type: Option<ValType> = None;
                                vv_iter.next().map(|id| {
                                    match id.expect_leaf("b") {
                                        TokenKind::Id(i) => {
                                            // idを入れる
                                            new_context.locals[param_idx] = Some(i);
                                        },
                                        TokenKind::ValType(vt) => {
                                            val_type = Some(vt);
                                        },
                                        _ => { panic!("paramの宣言が不正"); }
                                    }
                                });
                                param_idx += 1;

                                if val_type.is_none() {
                                    vv_iter.next().map(|elem| {
                                        if let TokenKind::ValType(vt) = elem.expect_leaf("") {
                                            val_type = Some(vt);
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
                    if child_cst.expect_leaf("") == TokenKind::FuncResult {
                        num_of_result = 1;
                        result_type = vv_iter.next().map(|type_cst| {
                            if let TokenKind::ValType(vt) = type_cst.expect_leaf("") {
                                vt
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
            // println!("new_context: {:?}", new_context.locals);
            (n, new_context, next)
        })
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
            if let Some(tk!(t)) = v_iter.next() {
                let funcidx = make_funcidx(t.clone(), context);
                Some(ExportDesc::Func(funcidx))
            } else {
                panic!("export funcの次が不正");
            }
        },
        Some(tk!(TokenKind::Table)) => {
            if let Some(tk!(t)) = v_iter.next() {
                let tableidx = make_tableidx(t.clone(), context);
                Some(ExportDesc::Table(tableidx))
            } else {
                panic!("export tableの次が不正");
            }
        },
        Some(tk!(TokenKind::Memory)) => {
            if let Some(tk!(t)) = v_iter.next() {
                let memidx = make_memidx(t.clone(), context);
                Some(ExportDesc::Mem(memidx))
            } else {
                panic!("export memoryの次が不正");
            }
        },
        Some(tk!(TokenKind::Global)) => {
            if let Some(tk!(t)) = v_iter.next() {
                let globalidx = make_globalidx(t.clone(), context);
                Some(ExportDesc::Global(globalidx))
            } else {
                panic!("export globalの次が不正");
            }
        },
        _ => None,
    }
}

macro_rules! make_idx_func {
    ($token:ident,$ret:ident,$v:expr) => {{
        match $token {
            TokenKind::Symbol(s) => {
                s.parse::<$ret>()
                .expect("idxがu32に変換できない")
            }
            TokenKind::Id(n) => {
                $v.iter().position(|tp| {
                    match tp {
                        Some(id) => id == &n,
                        _ => false,
                    }
                })
                .expect("contextから要素名が見つからない") as $ret
            }
            _ => { panic!("idxとして解釈不可"); }
        }
    }};
}

fn make_typeidx(token: TokenKind, context: &Context) -> TypeIndex {
    make_idx_func!(token, TypeIndex, context.types)
}

fn make_funcidx(token: TokenKind, context: &Context) -> FuncIndex {
    make_idx_func!(token, FuncIndex, context.funcs)
}

fn make_tableidx(token: TokenKind, context: &Context) -> TableIndex {
    make_idx_func!(token, TableIndex, context.tables)
}

fn make_memidx(token: TokenKind, context: &Context) -> MemIndex {
    make_idx_func!(token, MemIndex, context.mems)
}

fn make_globalidx(token: TokenKind, context: &Context) -> GlobalIndex {
    make_idx_func!(token, GlobalIndex, context.globals)
}

fn make_start(cst: &CST, context: &Context) -> Option<Start> {
    // println!("make_start cst: {:?}", cst);
    let mut start = Start::default();

    let v = cst.expect_node("");
    let mut v_iter = v.iter();
    v_iter.next();  // cstの0番目はstartだが、すでにチェック済みなので無視

    if let Some(tk!(t)) = v_iter.next() {
        let funcidx = make_funcidx(t.clone(), context);
        start.func = funcidx;
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
    if let Some(tk!(t)) = v_iter.next() {
        let tableidx = make_tableidx(t.clone(), context);
        elem.table = tableidx;
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
        if let tk!(t) = token {
            let funcidx = make_funcidx(t.clone(), context);
            elem.init.push(funcidx);
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
    if let Some(tk!(t)) = v_iter.next() {
        let memidx = make_memidx(t.clone(), context);
        data.data = memidx;
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

fn make_instrs<'a>(v: &mut (impl Iterator<Item=&'a CST>), context: &Context) -> Option<Vec<Instr>> {
    let mut instrs = vec![];
    let mut v = v.peekable();
    let mut instr = v.next();
    loop {
        println!("instr: {:?}", instr);
        if let Some(tk!(token)) = instr {
            if let Some(instr) = make_block_instr(token.clone(), context) {
                instrs.push(instr);
            }
            if let Some(instr) = make_plain_instr(token.clone(), context) {
                instrs.push(instr);
            }
        }
        if v.peek().is_none() { break; }
        instr = v.next();
    }

    Some(instrs)
}

fn make_block_instr(token: TokenKind, context: &Context) -> Option<Instr> {
    match token {
        // Control Instructions
        TokenKind::Block => {},
        TokenKind::Loop => {},
        TokenKind::If => {},
        _ => {},
    }
    None
}

fn make_plain_instr(token: TokenKind, context: &Context) -> Option<Instr> {
    match token {
        // Control Instructions
        TokenKind::Unreachable => {},
        TokenKind::Nop => {},
        TokenKind::Br => {},
        TokenKind::BrIf => {},
        TokenKind::BrTable => {},
        TokenKind::Return => {},
        TokenKind::Call => {
            // let n = csts[pos+1].expect_symbol("Instr call funcidxが取れない");
            // let res = n.parse::<u32>().map(|nn| {
            //     instrs.push(Instr::Call(nn));
            //     pos += 1;
            // });
        },
        TokenKind::CallIndirect => {},

        // Parametric Instructions
        TokenKind::Drop => {},
        TokenKind::Select => {},

        // Variable Instructions
        TokenKind::GetLocal => {},
        TokenKind::SetLocal => {},
        TokenKind::TeeLocal => {},
        TokenKind::GetGlobal => {},
        TokenKind::SetGlobal => {},

        // Memory Instructions
        TokenKind::I32Load => {},
        TokenKind::I64Load => {},
        TokenKind::F32Load => {},
        TokenKind::F64Load => {},
        TokenKind::I32Load8s => {},
        TokenKind::I32Load8u => {},
        TokenKind::I32Load16s => {},
        TokenKind::I32Load16u => {},
        TokenKind::I64Load8s => {},
        TokenKind::I64Load8u => {},
        TokenKind::I64Load16s => {},
        TokenKind::I64Load16u => {},
        TokenKind::I64Load32s => {},
        TokenKind::I64Load32u => {},
        TokenKind::I32Store => {},
        TokenKind::I64Store => {},
        TokenKind::F32Store => {},
        TokenKind::F64Store => {},
        TokenKind::I32Store8 => {},
        TokenKind::I32Store16 => {},
        TokenKind::I64Store8 => {},
        TokenKind::I64Store16 => {},
        TokenKind::I64Store32 => {},
        TokenKind::MemorySize => {},
        TokenKind::MemoryGrow => {},

        // Numeric Instructions
        TokenKind::I32Const => {
            // let val = v.next();
            // val.map(|cst| {
            //     let num_str = cst.expect_symbol("i32.const 定数値が取れない");
            //     let _ = num_str.parse::<u32>().map(|i32_num| {
            //         instrs.push(Instr::I32Const(Val::I32Const(i32_num)));
            //     });
            // });
        },
        TokenKind::I64Const => {},
        TokenKind::F32Const => {},
        TokenKind::F64Const => {},

        TokenKind::I32Clz => {},
        TokenKind::I32Ctz => {},
        TokenKind::I32Popcnt => {},
        TokenKind::I32Add => {},
        TokenKind::I32Sub => {},
        TokenKind::I32Mul => {},
        TokenKind::I32DivS => {},
        TokenKind::I32DivU => {},
        TokenKind::I32RemS => {},
        TokenKind::I32RemU => {},
        TokenKind::I32And => {},
        TokenKind::I32Or => {},
        TokenKind::I32Xor => {},
        TokenKind::I32Shl => {},
        TokenKind::I32ShrS => {},
        TokenKind::I32ShrU => {},
        TokenKind::I32Rotl => {},
        TokenKind::I32Rotr => {},

        TokenKind::I64Clz => {},
        TokenKind::I64Ctz => {},
        TokenKind::I64Popcnt => {},
        TokenKind::I64Add => {},
        TokenKind::I64Sub => {},
        TokenKind::I64Mul => {},
        TokenKind::I64DivS => {},
        TokenKind::I64DivU => {},
        TokenKind::I64RemS => {},
        TokenKind::I64RemU => {},
        TokenKind::I64And => {},
        TokenKind::I64Or => {},
        TokenKind::I64Xor => {},
        TokenKind::I64Shl => {},
        TokenKind::I64ShrS => {},
        TokenKind::I64ShrU => {},
        TokenKind::I64Rotl => {},
        TokenKind::I64Rotr => {},

        TokenKind::F32Abs => {},
        TokenKind::F32Neg => {},
        TokenKind::F32Ceil => {},
        TokenKind::F32Floor => {},
        TokenKind::F32Trunc => {},
        TokenKind::F32Nearest => {},
        TokenKind::F32Sqrt => {},
        TokenKind::F32Add => {},
        TokenKind::F32Sub => {},
        TokenKind::F32Mul => {},
        TokenKind::F32Div => {},
        TokenKind::F32Min => {},
        TokenKind::F32Max => {},
        TokenKind::F32CopySign => {},

        TokenKind::F64Abs => {},
        TokenKind::F64Neg => {},
        TokenKind::F64Ceil => {},
        TokenKind::F64Floor => {},
        TokenKind::F64Trunc => {},
        TokenKind::F64Nearest => {},
        TokenKind::F64Sqrt => {},
        TokenKind::F64Add => {},
        TokenKind::F64Sub => {},
        TokenKind::F64Mul => {},
        TokenKind::F64Div => {},
        TokenKind::F64Min => {},
        TokenKind::F64Max => {},
        TokenKind::F64CopySign => {},

        TokenKind::I32Eqz => {},
        TokenKind::I32Eq => {},
        TokenKind::I32Ne => {},
        TokenKind::I32LtS => {},
        TokenKind::I32LtU => {},
        TokenKind::I32GtS => {},
        TokenKind::I32GtU => {},
        TokenKind::I32LeS => {},
        TokenKind::I32LeU => {},
        TokenKind::I32GeS => {},
        TokenKind::I32GeU => {},

        TokenKind::I64Eqz => {},
        TokenKind::I64Eq => {},
        TokenKind::I64Ne => {},
        TokenKind::I64LtS => {},
        TokenKind::I64LtU => {},
        TokenKind::I64GtS => {},
        TokenKind::I64GtU => {},
        TokenKind::I64LeS => {},
        TokenKind::I64LeU => {},
        TokenKind::I64GeS => {},
        TokenKind::I64GeU => {},

        TokenKind::F32Eq => {},
        TokenKind::F32Ne => {},
        TokenKind::F32Lt => {},
        TokenKind::F32Gt => {},
        TokenKind::F32Le => {},
        TokenKind::F32Ge => {},

        TokenKind::F64Eq => {},
        TokenKind::F64Ne => {},
        TokenKind::F64Lt => {},
        TokenKind::F64Gt => {},
        TokenKind::F64Le => {},
        TokenKind::F64Ge => {},

        TokenKind::I32WrapToI64 => {},
        TokenKind::I32TruncSToF32 => {},
        TokenKind::I32TruncUToF32 => {},
        TokenKind::I32TruncSToF64 => {},
        TokenKind::I32TruncUToF64 => {},
        TokenKind::I64ExtendSToI32 => {},
        TokenKind::I64ExtendUToI32 => {},
        TokenKind::I64TruncSToF32 => {},
        TokenKind::I64TruncUToF32 => {},
        TokenKind::I64TruncSToF64 => {},
        TokenKind::I64TruncUToF64 => {},
        TokenKind::F32ConvertSToI32 => {},
        TokenKind::F32ConvertUToI32 => {},
        TokenKind::F32ConvertSToI64 => {},
        TokenKind::F32ConvertUToI64 => {},
        TokenKind::F32DemoteToF64 => {},
        TokenKind::F64ConvertSToI32 => {},
        TokenKind::F64ConvertUToI32 => {},
        TokenKind::F64ConvertSToI64 => {},
        TokenKind::F64ConvertUToI64 => {},
        TokenKind::F64PromoteToF32 => {},
        TokenKind::I32ReinterpretToF32 => {},
        TokenKind::I64ReinterpretToF64 => {},
        TokenKind::F32ReinterpretToI32 => {},
        TokenKind::F64ReinterpretToI64 => {},

        _ => {},
    }

    None
}