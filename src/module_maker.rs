use core::{Token, Tree, CST, Module, Func, FuncType, TypeUse, Import, ImportDesc, Instr, Start
};

macro_rules! err {
    ($message: expr) => {
        println!($message);
        return None;                        
    };
}

pub fn make_module(cst: CST) -> Option<Module> {
    let mut module = Module::new();

    let v = cst.expect_node("make_module cst is not node");
    let vv = v[0].expect_node("make_module v is not node");
    let mut pos = 0;
    
    if vv.len() > 0 && vv[pos] == Tree::Leaf(Token::Module) {
        pos += 1;
        // 通常のモジュール                        

        // module id なくてもいい
        if let Tree::Leaf(Token::Name(n)) = &vv[1] {
            module.id = Some(n.clone());
            pos += 1;
        }

        for module_field in &vv[pos..] {
            // println!("module_field: {:?}", module_field);                            
            let vvv = module_field.expect_node("module読込時エラー");

            match &vvv[0].expect_leaf("module nodeの最初がkeywordではない") {                                    
                Token::Type => {
                    let pos = 1;
                    // type id は一旦無視

                    make_functype(&vvv[pos]).map(|functype| {
                        module.types.push(functype);
                    });
                },

                Token::Import => {
                    let import = make_import(&module_field).expect("import作れず");
                    module.imports.push(import);
                },

                Token::Func => {
                    make_func(&module_field).map(|func| {
                        module.funcs.push(func);
                    });
                },

                Token::Start => { module.start = make_start(&module_field); },

                _ => {},
            }
        }

        return Some(module);
    }

    None    
}

fn make_functype(cst: &CST) -> Option<FuncType> {
    // println!("make_functype cst: {:?}", cst);

    let mut param_types = vec![];
    let mut result_types = vec![];

    if let Tree::Node(v) = cst {
        if v[0] != Tree::Leaf(Token::Func) {
            println!("直下はFuncから始まる");
            return None;
        }

        // funcのidは一回無視
        
        let mut result_scanned = false;
        for node in &v[1..] {
            if let Tree::Node(v) = node {
                match v[0] {
                    Tree::Leaf(Token::Param) => {
                        if result_scanned {                        
                            println!("resultの後にparamは置けない");
                            return None;
                        }

                        // param id は一旦無視

                        if let Tree::Leaf(Token::ValType(vt)) = &v[1] {
                            param_types.push(vt.clone());
                        } else {
                            println!("パラメータの型名が正しくない");
                            return None;
                        }
                    },
                    Tree::Leaf(Token::FuncResult) => {
                        result_scanned = true;                    

                        if let Tree::Leaf(Token::ValType(vt)) = &v[1] {
                            result_types.push(vt.clone());
                        } else {
                            println!("戻り値の型名が正しくない");
                            return None;
                        }
                    },
                    _ => {},
                }
            } else {
                return None;
            }
        }

    } else {
        println!("typeの次がNodeじゃない");
        return None;
    }

    Some((param_types, result_types))
}

macro_rules! slice_get {
    ($v:ident, $s:ident, $i:expr, $func_name:expr) => {
        let $v = $s.get($i).expect(&format!("{} {}.get({})", $func_name, stringify!($s), stringify!($i)));
    };
}

// import ::= {module name, name name, desc importdesc}
// importdesc ::= func typeidx
//              | table tabletype
//              | mem memtype
//              | global globaltype

// まずは以下を
// (import "wasi" "print" (func (type 0)))
// 
fn make_import(cst: &CST) -> Option<Import> {
    println!("make_import cst: {:?}", cst);

    let v = cst.expect_node("make_import not node");

    let mut imp = Import::default();
    slice_get!(v1, v, 1, "make_import");
    imp.module_name = v1.expect_text("make_import モジュール名が取れず");    
    imp.element_name = v[2].expect_text("make_import 要素名が取れず");

    make_import_desc(&v[3]).map(|imp_desc| {
        imp.desc = imp_desc;
        imp
    })
}

fn make_import_desc(cst: &CST) -> Option<ImportDesc> {
    let mut imp_desc = ImportDesc::default();

    let desc_node = cst.expect_node("make_import_desc 要素が取れない");
    match desc_node[0].expect_leaf("make_import_desc 要素の最初が間違ってる") {
        Token::Func => {
            make_typeuse(&desc_node[1]).map(|typeuse| {
                imp_desc.func = typeuse;
            });
        },
        _ => {},
    }

    Some(imp_desc)
}

fn make_typeuse(cst: &CST) -> Option<TypeUse> {
    let typeuse = cst.expect_node("make_typeuse typeuseが取れない");
    let s = typeuse[1].expect_symbol("make_typeuse funcidx取れない");
    let n = s.parse::<u32>().expect("make_typeuse funcidxがu32に変換できない");
    Some(TypeUse(n))
}

fn make_func(cst: &CST) -> Option<Func> {
    // println!("make_func cst: {:?}", cst);
    let mut func = Func::new();

    let v = cst.unwrap_node();
    // cstの0番目はFuncだが、すでにチェック済みなので無視

    let mut pos = 1;

    // Funcのid（チェックするがまだ使わない
    if let Tree::Leaf(Token::Name(_)) = v[pos] {
        pos += 1;
    }

    if v.len() <= pos { err!("Funcないの数が正しくない"); }

    // type
    // ひとまず、省略記法のことは考えないようにしよう
    make_typeuse(&v[pos]).map(|typeuse| {
        func.func_type = typeuse;
        pos += 1;
    });

    // ローカル変数宣言
    loop {
        if v.len() <= pos { break; }

        // println!("make_func elem: {:?}", v[pos]);
        if let Tree::Node(local_variable) = &v[pos] {
            
            if local_variable[0].match_token(Token::Local) {
                let vt = local_variable[1].expect_valtype("make_func ローカル宣言で'local'の後にValTypeがこない");
                func.locals.push(vt);
            } else {
                err!("make_func ローカル宣言で'local'から始まってない");
            }
        } else {
            break;
        }

        pos += 1;
    }                

    // isntr
    loop {
        if v.len() <= pos { break; }

        make_instr(&v[pos..]).map(|instrs| {
            func.body.instrs = instrs;
        });

        pos += 1;        
        break;
    }

    Some(func)
}

fn make_instr(csts: &[CST]) -> Option<Vec<Instr>> {
    // println!("make_instr cst: {:?}", csts);

    let mut instrs = vec![]; 

    let mut pos = 0;
    loop {
        if csts.len() <= pos { break; }

        match &csts[pos] {
            Tree::Leaf(t) => {
                match t {
                    Token::Call => {
                        let n = csts[pos+1].expect_symbol("Instr call funcidxが取れない");
                        let res = n.parse::<u32>().map(|nn| {
                            instrs.push(Instr::Call(nn));
                            pos += 1;
                        });
                    },
                    _ => {},
                }                
            },
            Tree::Node(_) => {
                // fold exprはまだ
            },
        }        

        pos += 1;
    }

    Some(instrs)
}

fn make_start(cst: &CST) -> Option<Start> {
    println!("make_start cst: {:?}", cst);
    let mut start = Start::default();

    let v = cst.expect_node("make_start 要素が取れない");
    // v[0]はチェック済み
    let n = v[1].expect_symbol("make_start funcidx取れない");
    start.func = n.parse::<u32>().expect("make_start funcidxが数値じゃない");
    
    Some(start)
}