use std::io::{Read, Seek};
use super::*;

impl Lexer {

pub fn lex_string(&mut self, reader: &mut (impl Read + Seek)) -> LexerResult {

    let mut string = vec![];
    let mut string_c = self.read(reader)?;
    let mut byte_length_of_codepoint = 0;  // 1 ~ 3
    let mut rest_of_byte_of_char = 0;  // 0 ~ 3
    loop {
        match string_c {
            // end of string
            b'"' => { self.loc.add_pos(); break; },
            // escape sequence
            b'\\' => {
            },
            _ if string_c > 0x20 && string_c != 0x7F => {
                if rest_of_byte_of_char == 0 {
                    // count byte as codepoint (not utf-8 bit pattern)
                    match string_c {
                        0x00 ... 0x7F => self.loc.add_pos(),
                        0xC2 ... 0xDF => {                                        
                            rest_of_byte_of_char = 1;
                            byte_length_of_codepoint = 2;
                        },
                        0xE0 ... 0xEF => {
                            rest_of_byte_of_char = 2;
                            byte_length_of_codepoint = 2;                                        
                        },
                        0xF0 ... 0xF7 => {
                            rest_of_byte_of_char = 3;
                            byte_length_of_codepoint = 3;                                        
                        },
                        _ => return Err(LexError::invalid_char(string_c, self.loc.clone())),
                    }
                } else {
                    match string_c {
                        0x80 ... 0xBF => {
                            rest_of_byte_of_char -= 1;
                            if rest_of_byte_of_char == 0 {
                                for _ in 0..byte_length_of_codepoint {
                                    self.loc.add_pos();
                                }
                            }
                        }
                        _ => return Err(LexError::invalid_char(string_c, self.loc.clone())),
                    }
                }
                string.push(string_c);
            },
            _ => return Err(LexError::invalid_char(string_c, self.loc.clone())),
        }
        string_c = self.read(reader)?;
    }
    let res = String::from_utf8(string.to_vec())?;

    self.current = self.read(reader)?;
    return Ok(Token::string(res, self.loc.clone()))
}

}