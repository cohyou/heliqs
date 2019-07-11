// Cursor<&[u8]>

mod wannaiter;

use std::fs::File;
use std::io::prelude::*;

fn main() {    
    let file_name = "a.wat";
    let mut file = File::open(file_name).unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("pppp{}", s);

    // use std::io::BufRead;
    let stdin = std::io::BufReader::new(std::io::stdin());
    let mut lines = stdin.lines();

    let mut mode = LexingMode::Normal;
    let mut tokens: Vec<String> = vec![];
    let mut current_token: Vec<u8> = vec![];

    loop {
        if let Some(Ok(line)) = lines.next() {
            for b in line.into_bytes() {
                match b {
                    b'(' => {
                        // かっこ始まり
                        tokens.push("(".to_string());
                        current_token.clear();
                    },
                    b')' => {
                        // かっこ終わり
                        tokens.push(")".to_string());
                        current_token.clear();
                    },
                    _ if is_whitespace(b) => {
                        if current_token.len() > 0 {
                            tokens.push(get_token_string(&current_token));
                            current_token.clear();
                        }                                                
                    },
                    _ => {
                        current_token.push(b);
                    },
                }
            }
        } else {
            tokens.push(get_token_string(&current_token)); 
            break;
        }
    }    

    let mut current_token_index: usize = 0;
    // let tokens = get_tokens(lin]es as std::iter::Iterator<Item=String>);
    if tokens[0] == "(" && tokens[1] == "module" {        
        println!("OK");
        current_token_index = 2;

        if tokens[current_token_index] == ")" {
            println!("モジュール終了");
        } else {
            println!("モジュールが正しく終了していない");
        }
    } else {
        println!("正しい形式ではない、moduleが最初");
    }

    // for t in tokens {
    //     println!("{:?}", t);
    //     match t.as_ref() {
    //         "(" => {
    //             println!("www");
    //         },
    //         _ => println!("wwwaahw4s5")
    //     }
    //     println!("{:?}", parse_module());
    // }

    let wann = wannaiter::WannaIter{ i: 1 };
    for w in wann {
        println!("{:?}", w);
    }
}

fn parse_module(tokens: impl Iterator::<Item=String>, token_index: usize) -> Module {
    Module {}
}

fn parse_func(tokens: impl Iterator::<Item=String>, token_index: usize) {
    // if tokens[token_index] == "(" {
    //     ()
    // } else {
    //     ()
    // }
    ()
}

fn is_whitespace(b: u8) -> bool {
    b == b' '
}

enum LexingMode {
    Normal,
}

fn get_token_string(token_bytes: &Vec<u8>) -> String {
    String::from_utf8(token_bytes.clone()).expect("Found invalid UTF-8")
}

#[derive(Debug)]
struct Module;

// use std::io::Lines;

// fn get_tokens(lines: Lines<std::io::Stdin>) -> Vec<String> {
//     tokens
// }

fn read_bytes<R: Read>(reader: &mut R) {
    while let Some(Ok(c)) = reader.bytes().next() {
        println!("{:?}", c);
    }
}

#[test]
fn test1() {
    let file_name = "a.wat";
    let mut file = File::open(file_name).unwrap();
    read_bytes(&mut file);
}

#[test]
fn test2() {
    use std::io::Cursor;
    let a = "2tv4ghq73arfe@djim:o;,";
    let mut cur1 = Cursor::new(a);
    read_bytes(&mut cur1);
}
