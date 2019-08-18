mod error;
#[macro_use]mod comment;
#[macro_use]mod keyword;
mod string;
mod token;

use std::io::{Read, Seek};
use annot::{Loc};

pub use self::error::*;
pub use self::comment::*;
pub use self::keyword::*;
pub use self::string::*;
pub use self::token::*;

#[derive(Debug)]
pub struct Lexer {
    pub current: u8,
    pub loc: Loc,
}

pub type LexerResult = Result<Token, LexError>;

impl Lexer {

pub fn new(reader: &mut (impl Read + Seek)) -> Lexer {
    let loc = Loc::default();
    let mut buf: &mut [u8] = &mut [0;1];
    let n = reader.read(&mut buf).unwrap();
    if n == 0 {
        Lexer { current: 0xFF, loc: loc }
    } else {
        Lexer { current: buf[0], loc: loc }
    }

}

pub fn lex_token(&mut self, reader: &mut (impl Read + Seek)) -> LexerResult {

    loop {
        match self.current {
            // space (normal delimiter)
            b'\t' | b' ' => {
                self.loc.add_pos();
            },

            // space (LF)
            b'\n' => {
                self.loc.newline();
            },

            // space (CR)
            b'\r' => {},

            // line comment
            b';' => {
                self.loc.add_pos();
                lex_line_comment!(self, reader);
            },

            // keyword
            b'a' ... b'z' => {
                self.loc.add_pos();
                let new_loc = self.loc.clone();

                let mut keyword = vec![self.current];
                let mut keyword_c = self.read(reader)?;
                loop {
                    if is_idchar(keyword_c) {
                        self.loc.add_pos();
                        keyword.push(keyword_c);
                    } else {
                        self.current = keyword_c;
                        break;
                    }
                    keyword_c = self.read(reader)?;
                }

                return vec_to_keyword(keyword.as_slice())
                .map_or(Ok(Token::reserved(keyword, new_loc.clone())),
                |kw| Ok(Token::keyword(kw, new_loc)))
            },

            // num or hexnum (uN)
            b'0' ... b'9' => {
                self.loc.add_pos();

                let mut un_c = self.current;
                if self.current == b'0' {
                    un_c = self.read(reader)?;
                    if un_c == b'x' {
                        self.loc.add_pos();
                        // hexnum
                        self.current = self.read(reader)?;
                        return Ok(Token::number_u(0, self.loc.clone()))
                    }
                }

                // num
                let mut num = 0;
                let mut num_c = un_c;
                loop {
                    match num_c {
                        b'_' => self.loc.add_pos(),
                        b'0' => { self.loc.add_pos(); num = num * 10 + 0; },
                        b'1' => { self.loc.add_pos(); num = num * 10 + 1; },
                        b'2' => { self.loc.add_pos(); num = num * 10 + 2; },
                        b'3' => { self.loc.add_pos(); num = num * 10 + 3; },
                        b'4' => { self.loc.add_pos(); num = num * 10 + 4; },
                        b'5' => { self.loc.add_pos(); num = num * 10 + 5; },
                        b'6' => { self.loc.add_pos(); num = num * 10 + 6; },
                        b'7' => { self.loc.add_pos(); num = num * 10 + 7; },
                        b'8' => { self.loc.add_pos(); num = num * 10 + 8; },
                        b'9' => { self.loc.add_pos(); num = num * 10 + 9; },
                        0xFF => return Err(LexError::eof(&self.loc)),
                        _ => break,
                    }
                    num_c = self.read(reader)?;
                }

                self.current = num_c;
                return Ok(Token::number_u(num, self.loc.clone()))
            },

            // number (sN or fN)
            b'+' | b'-' => return Ok(Token::number_u(0, self.loc.clone())),

            // string
            b'"' => {
                self.loc.add_pos();
                return self.lex_string(reader);
            },

            // id        
            b'$' => {
                self.loc.add_pos();

                let new_loc = self.loc.clone();

                let mut id = vec![];
                let mut id_c = self.read(reader)?;
                loop {
                    if is_idchar(id_c) {
                        self.loc.add_pos();
                        id.push(id_c);
                    } else {
                        self.current = id_c;
                        break;
                    }
                    id_c = self.read(reader)?;
                }

                let res = String::from_utf8(id.to_vec())?;                
                return Ok(Token::id(res, new_loc))
            },  

            // left paren or start of block comment
            b'(' => {
                self.loc.add_pos();
                let c = self.read(reader)?;

                if c != b';' {
                    // left paren
                    self.current = c;
                    return Ok(Token::left_paren(self.loc.clone()));
                }
                self.loc.add_pos();

                // block comment
                self.lex_block_comment(reader)?;
            },

            // right paren
            b')' => {
                self.loc.add_pos();
                self.current = self.read(reader)?;
                // println!("self.current: {:?}", self.current);
                return Ok(Token::right_paren(self.loc.clone()));
            },

            // reserved
            _ if is_idchar(self.current) => return Ok(Token::reserved(vec![], self.loc.clone())),

            // EOF
            0xFF => return Ok(Token::empty(self.loc.clone())),

            // invalid
            _ => return Err(LexError::invalid_char(self.current, self.loc.clone())),
        };

        self.current = self.read(reader)?;
    }
}

}

fn is_idchar(c: u8) -> bool {
    match c {
        b'0' ... b'9' |
        b'A' ... b'Z' |
        b'a' ... b'z' |
        b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'/' |
        b':' | b'<' | b'=' | b'>' | b'?' | b'@' | b'\\' | b'^' | b'_' | b'`' | b'|' | b'~' => true,
        _ => false,
    }
}

#[test]
fn test_lex_token() {
    use std::io::Cursor;
    // let mut reader = Cursor::new("\r  (; comment ;) (   module)");
    let mut reader = Cursor::new("(m)");    

    let mut lexer = Lexer::new(&mut reader);
    lexer.lex_token(&mut reader);
    assert_eq!(lexer.lex_token(&mut reader), Ok(Token::empty(lexer.loc)));
}