use core::{Token, Tree, CST, Module, Func, FuncType};

pub fn make_module(cst: CST) -> Option<Module> {
    let mut module = Module::new();

    match cst {
        Tree::Node(v) => {
            match &v[0] {
                Tree::Node(vv) => {
                    let mut pos = 0;
                    
                    if vv.len() > 0 && vv[pos] == Tree::Leaf(Token::Module) {
                        pos += 1;
                        // 通常のモジュール                        

                        if let Tree::Leaf(Token::Name(n)) = &vv[1] {
                            module.id = Some(n.clone());
                            pos += 1;
                        }

                        for module_field in &vv[pos..] {
                            // println!("module_field: {:?}", module_field);                            
                            if let Tree::Node(vvv) = module_field {
                                match &vvv[0] {                                    
                                    Tree::Leaf(Token::Type) => {
                                        let pos = 1;
                                        // type id は一旦無視

                                        if let Some(functype) = make_functype(&vvv[pos]) {
                                            module.types.push(functype);
                                        }
                                    },
                                    Tree::Leaf(Token::Func) => {
                                        if let Some(func) = make_func(&module_field) {
                                            module.funcs.push(func);
                                        } else {
                                            // 何かfuncを読んでいる時にエラー
                                            ;
                                        }
                                    },
                                    _ => {},
                                }
                            } else {
                                // これは構文エラー。
                                // name以降のmodulefieldsは全てlistで構成されているはずなので
                                ;                            
                            }
                        }

                        return Some(module);
                    }
                },
                _ => {},
            }
        },
        _ => {},
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

fn make_func(cst: &CST) -> Option<Func> {
    println!("make_func cst: {:?}", cst);
    let func = Func::new();

    let v = cst.unwrap();
    // cstの0番目はFuncだが、すでにチェック済みなので無視

    let mut pos = 1;

    // Funcのid（チェックするがまだ使わない
    if let Tree::Leaf(Token::Name(_)) = v[1] {
        pos += 1;
    }

    for element in &v[pos..] {        
        println!("make_func element: {:?}", element);
    }

    Some(func)
}