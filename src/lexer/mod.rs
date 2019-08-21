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
pub struct Lexer<R: Read + Seek> {
    reader: R,
    current: u8,
    loc: Loc,
    peeked_byte: u8,
    peeked_token: Option<Token>,
}

pub type LexerResult = Result<Token, LexError>;

impl<R> Lexer<R> where R: Read + Seek {

pub fn new(mut reader: R) -> Lexer<R> {
    let loc = Loc::default();
    let mut buf: &mut [u8] = &mut [0;1];
    let n = reader.read(&mut buf).unwrap();
    let current = if n == 0 { 0xFF } else { buf[0] };

    Lexer {
        reader: reader, 
        current: current, 
        loc: loc, 
        peeked_byte: 0, 
        peeked_token: None 
    }
}

pub fn next_token(&mut self) -> LexerResult {
    if let Some(peeked) = &self.peeked_token {
        let result = peeked.clone();
        self.peeked_token = None;
        Ok(result)
    } else {
        self.next_token_internal()
    }
}

pub fn peek_token(&mut self) -> LexerResult {
    if self.peeked_token.is_none() {
        self.peeked_token = Some(self.next_token_internal()?);
    }
    let result = self.peeked_token.clone();
    Ok(result.unwrap())
}

fn next_token_internal(&mut self) -> LexerResult {

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
                lex_line_comment!(self, self.reader);
            },

            // keyword
            b'a' ... b'z' => {
                self.loc.add_pos();
                let new_loc = self.loc;

                let mut keyword = vec![self.current];
                let mut keyword_c = self.read()?;
                loop {
                    if is_idchar(keyword_c) {
                        self.loc.add_pos();
                        keyword.push(keyword_c);
                    } else {
                        self.current = keyword_c;
                        break;
                    }
                    keyword_c = self.read()?;
                }

                return vec_to_keyword(keyword.as_slice())
                .map_or(Ok(Token::reserved(keyword, new_loc)),
                |kw| Ok(Token::keyword(kw, new_loc)))
            },

            // num or hexnum (uN)
            b'0' ... b'9' => {

                if self.current == b'0' {
                    if self.peek()? == b'x' {
                        self.read()?;
                        self.loc.add_pos();  // for 0
                        self.loc.add_pos();  // for x
                        // hexnum
                        self.current = self.read()?;
                        return Ok(Token::number_u(0, self.loc))
                    }
                }

                // num
                let mut num = 0;
                let mut num_c = self.current;
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
                        0xFF => return Err(LexError::eof(self.loc)),
                        _ => break,
                    }
                    num_c = self.read()?;
                }

                self.current = num_c;
                return Ok(Token::number_u(num, self.loc))
            },

            // number (sN or fN)
            b'+' | b'-' => return Ok(Token::number_u(0, self.loc)),

            // string
            b'"' => {
                self.loc.add_pos();
                return self.lex_string();
            },

            // id
            b'$' => {
                self.loc.add_pos();

                let new_loc = self.loc;

                let mut id = vec![];
                let mut id_c = self.read()?;
                loop {
                    if is_idchar(id_c) {
                        self.loc.add_pos();
                        id.push(id_c);
                    } else {
                        self.current = id_c;
                        break;
                    }
                    id_c = self.read()?;
                }

                let res = String::from_utf8(id.to_vec())?;
                return Ok(Token::id(res, new_loc))
            },

            // left paren or start of block comment
            b'(' => {
                self.loc.add_pos();
                let c = self.read()?;

                if c != b';' {
                    // left paren
                    self.current = c;
                    return Ok(Token::left_paren(self.loc));
                }
                self.loc.add_pos();

                // block comment
                self.lex_block_comment()?;
            },

            // right paren
            b')' => {
                self.loc.add_pos();
                self.current = self.read()?;
                // println!("self.current: {:?}", self.current);
                return Ok(Token::right_paren(self.loc));
            },

            // reserved
            _ if is_idchar(self.current) => return Ok(Token::reserved(vec![], self.loc)),

            // EOF
            0xFF => return Ok(Token::empty(self.loc)),

            // invalid
            _ => return Err(self.err(self.current)),
        };

        self.current = self.read()?;
    }
}

fn read(&mut self) -> Result<u8, LexError> {
    if self.peeked_byte == 0 {
        self.read_internal()
    } else {
        let peeked = self.peeked_byte;
        self.peeked_byte = 0;
        Ok(peeked)
    }    
}

fn peek(&mut self) -> Result<u8, LexError> {
    if self.peeked_byte == 0 {
        self.peeked_byte = self.read_internal()?;
    }
    Ok(self.peeked_byte)
}

fn read_internal(&mut self) -> Result<u8, LexError> {
    let mut buf: &mut [u8] = &mut [0;1];
    let n = self.reader.read(&mut buf)?;

    if n == 0 { return Ok(0xFF) }
    Ok(buf[0])
}

fn err(&self, c: u8) -> LexError {
    LexError::invalid_char(c, self.loc)
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