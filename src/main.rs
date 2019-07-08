// バイト
struct Byte(u8);

// u32 u64 s32 s64 i8 i16 i32 i64
// 以上をひとまず対象とするが、本来はバイト数は自由らしい
struct Unsigned8(u8);
struct Unsigned16(u16);
struct Unsigned32(u32);
struct Unsigned64(u64);

struct Signed32(i32);
struct Signed64(i64);

type Integer8 = Unsigned8;
type Integer16 = Unsigned16;
type Integer32 = Unsigned32;
type Integer64 = Unsigned64;

// floating-points
struct Float32(f32);
struct Float64(f64);

// Nameはcodepointの列。
// これはstrかStringか
// Due to the limitations of the binary format, the length of a name is bounded by the length of its UTF-8 encoding.

// Convention
// Code points are sometimes used interchangeably with natural numbers n < 1114112.
// 1114112はたぶんU+10FFFFのこと。

// 簡単そうなStringにします
type Name = String;


// 2.3 Types

// 2.3.1 Value Types
// valtype　::= 'i32 | 'i64 | 'f32 | 'f64
enum ValType {
    Integer32,
    Integer64,
    Float32,
    Float64,
}

// 2.3.2 Result Types
// resulttype ::= '[ valtype? '] valtypeが0個か1個の列ということ？
struct ResultType(Option<ValType>);

// 2.3.3 Function Types
// functype ::= '[ vec(valtype) '] '-> '[ vec(valtype) ']
// これはどうやって宣言すればいいのだろう

// 2.3.4 Limits
// limits ::= '{ 'min u32, 'max u32? '}
// If no maximum is given, the respective storage can grow to any size.

// 2.3.5 Memory Types
// memtype  ::= limits

// 2.3.6 Table Types
// tabletype ::= limits elemtype
// elemtype ::= 'anyfunc

// 2.3.7 Global Types
// globaltype ::= mut valtype
// mut ::= 'const | 'var

// 2.3.8 External Types
// externtype ::= 'func functype | 'table tabletype | 'mem memtype | 'global globaltype
// ちなみに、externtypeの列から特定の種類のものだけを取り出すnotationを以下に定義する
// 順序は保存した状態で取り出す
// funcs(externtype*) = [functype | ('func functype) ∈ externtype*]
// tables(externtype*) = [tabletype | ('table tabletype) ∈ externtype*]
// mems(externtype*) = [memtype | ('mem memtype) ∈ externtype*]
// globals(externtype*) = [globaltype | ('global globaltype) ∈ externtype*]

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

use std::io::Lines;

// fn get_tokens(lines: Lines<std::io::Stdin>) -> Vec<String> {
//     tokens
// }

fn main() {
    use std::io::BufRead;

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
}

fn parse_module(tokens: impl Iterator::<Item=String>, token_index: usize) -> Module {
    Module {}
}

fn parse_func(tokens: impl Iterator::<Item=String>, token_index: usize) {
    if tokens[token_index] == "(" {
        ()
    } else {
        ()
    }
}

// イテレータを自分で作りたい！