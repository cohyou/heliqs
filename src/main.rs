mod core;
mod lexer;
mod parser;
mod module_maker;
mod runtime;

use std::fs::File;

use core::*;
use runtime::*;

fn main() {
    let file_name = "a.wat";
    let mut file = File::open(file_name).unwrap();

    // while let Some(t) = lexer::lex(&mut file) {
    //     println!("{:?}", t);
    // }
    let mut p = parser::Parser::new();
    if let Some(cst) = p.parse(&mut file) {
        // println!("CST:    {:?}", cst);
        if let Some(module) = module_maker::make_module(cst) {
            println!();
            println!("---- ---- RESULT ---- ----");
            println!();
            println!("Module: {:?}", module.id);
            println!("types: {:?}", module.types);
            println!("imports: {:?}", module.imports);            
            println!("funcs:");
            for func in &module.funcs {
                println!("    {:?}", func);
            }
            println!("start: {:?}", module.start);

            println!();
            let mut store = Runtime::init_store();
            let func_inst = FuncInst::Host { func_type: (vec![ValType::I32], vec![]), host_code: "log".to_string() };
            store.insts.push(StoreInst::Func(func_inst));
            
            let mut rt = Runtime::new(Some(store));
            let extern_vals = vec![ExternVal::Func(0)];
            println!("module instance: {:?}", rt.instantiate(&module, extern_vals));
        }
    }    
}

// Option<T>のコンビネータ

// 1, 単体のOption値に関するコンビネータ
// 普通は、パターンマッチを行って、中の値を取り出す
// if let
// match

// むりやり取る（ダメならpanic
// unwrap()

// ダメならメッセージ付きでpanic
// expect(メッセージ)

// unwrap_or(代わりの値)
// ダメなら代わりの値を返す

// unwrap_or_else（クロージャ）
// ダメなら代わりの値をクロージャで計算して返す

// 2, 複数のOption値をつなげるコンビネータ

// 例：map(|s| s.len())
// NoneならNoneが伝播する、Someなら中身がsに渡る

// map_or
// mapもあるし、Noneだった時に返す値も決められる

// map_or_else（クロージャ）
// map_orのクロージャ版

// and
// x.and(y)でxとy両方がSomeならyを返す
// そうでなければNone
// mapは前の値が渡ってくるが、andは前の値に関係ない場合に使える

// and_then
// mapとよく似ているが、戻り値の型が違う
// mapは中身を返せば自動的にSomeで包んでくれる
// and_thenはOption<U>を返すクロージャを渡す
// Noneが伝播するのは同じ

// or
// x.or(y) でショートサーキットのある方のSomeを返す

// or_else
// and_thenのor版
// 名前が紛らわしい、

// TがDefaultを実装していれば、
// unwrap_or_default()も使える。


// ok_or
// OptionをErrorに変える
// Some("foo").ok_or(エラー) => Ok("foo")
// None       .ok_or(エラー) => Err(エラー)

// ok_or_else
// ok_orのクロージャ版
