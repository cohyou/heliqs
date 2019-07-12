use std::io::{Read, Seek, SeekFrom};
use token::Token;

macro_rules! make_token {
    ($bytes:ident) => {
        if !$bytes.is_empty() {
            Some(Token::Name(String::from_utf8($bytes).unwrap()))
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
                    b'(' => { return Some(Token::LeftParen); },
                    b')' => { return Some(Token::RightParen); },
                    b' ' => {},
                    _ => {
                        token_bytes.push(c[0]);
                        loop {
                            if let Ok(_) = reader.read(&mut c) {
                                match c[0] {
                                    b'(' | b')' | b' ' => {
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