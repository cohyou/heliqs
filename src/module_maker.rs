use core::{Token, Tree, CST, Module, Func};

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

                        for module_field in &vv[2..] {
                            println!("module_field: {:?}", module_field);                            
                            if let Tree::Node(vvv) = module_field {
                                match &vvv[0] {                                    
                                    Tree::Leaf(Token::Func) => {
                                        if let Some(func) = make_func() {
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

                        println!("wowow: {:?}", vv);

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

fn make_func() -> Option<Func> {
    None
}