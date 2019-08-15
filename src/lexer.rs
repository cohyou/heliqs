use std::io::{Read, Seek, SeekFrom};
use core::{Annot, Loc, Token, ValType};

macro_rules! make_token {
    ($bytes:ident) => {
        if !$bytes.is_empty() {
            let s = String::from_utf8($bytes.to_vec()).unwrap();
            let t = match s.as_ref() {
                "module" => Token::module(Loc(0, 0)),

                "type" => Token::func_type(Loc(0, 0)),
                "import" => Token::import(Loc(0, 0)),
                "func" => Token::func(Loc(0, 0)),
                "table" => Token::table(Loc(0, 0)),
                "memory" => Token::memory(Loc(0, 0)),
                "global" => Token::global(Loc(0, 0)),
                "export" => Token::export(Loc(0, 0)),
                "start" => Token::start(Loc(0, 0)),
                "elem" => Token::elem(Loc(0, 0)),
                "data" => Token::data(Loc(0, 0)),

                "local" => Token::local(Loc(0, 0)),
                "param" => Token::param(Loc(0, 0)),
                "result" => Token::func_result(Loc(0, 0)),
                "anyfunc" => Token::any_func(Loc(0, 0)),
                "mut" => Token::mutable(Loc(0, 0)),
                "offset" => Token::offset(Loc(0, 0)),

                "i32" => Token::val_type(ValType::I32, Loc(0, 0)),
                "i64" => Token::val_type(ValType::I64, Loc(0, 0)),
                "f32" => Token::val_type(ValType::F32, Loc(0, 0)),
                "f64" => Token::val_type(ValType::F64, Loc(0, 0)),

                "i32.const" => Token::i32_const(Loc(0, 0)),

                "call" => Token::call(Loc(0, 0)),

                _ if $bytes[0] == b'$' => Token::id(s[1..].to_string(), Loc(0, 0)),
                _ => {
                    Token::symbol(s, Loc(0, 0))
                },
            };
            Some(t)
        } else {
            None
        }
    };
}

pub fn lex(reader: &mut (impl Read + Seek)) -> Option<Token> {
    let mut c: &mut [u8] = &mut [0;1];
    let mut token_bytes: Vec<u8> = vec![];

    loop {
        if let Ok(n) = reader.read(&mut c) {
            if n > 0 {
                // println!("c: {:?}", c);
                match c[0] {
                    b'(' => { return Some(Token::left_paren(Loc(0, 0))); },
                    b')' => { return Some(Token::right_paren(Loc(0, 0))); },
                    b' ' | b'\n' => {},
                    b'\"' => {
                        // stringの開始
                        while reader.read(&mut c).expect("lex: EOF") > 0 && c[0] != b'\"' {
                            token_bytes.push(c[0])
                        }
                        let s = String::from_utf8(token_bytes.to_vec()).unwrap();
                        return Some(Token::text(s, Loc(0, 0)))
                    },
                    _ => {
                        token_bytes.push(c[0]);
                        return lex_chars(reader, &mut token_bytes);
                    },
                }
            } else {
                return make_token!(token_bytes);
            }
        } else {
            // 本当はエラーを返したほうがいい
            return None;
        }
    }
}

fn lex_chars(reader: &mut (impl Read + Seek), token_bytes: &mut Vec<u8>) -> Option<Token> {
    let mut c: &mut [u8] = &mut [0;1];
    loop {
        if let Ok(_) = reader.read(&mut c) {
            match c[0] {
                b'(' | b')' | b' ' | b'\n' => {
                    reader.seek(SeekFrom::Current(-1)).unwrap();
                    return make_token!(token_bytes);
                },
                _ => {
                    token_bytes.push(c[0]);
                }
            }
        } else {
            // 本当はエラーを返したほうがいい
            return None;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }

    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}