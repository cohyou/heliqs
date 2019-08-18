use std::io::{Read, Seek};
use super::error::*;
use super::Lexer;

// <R>(reader: &mut R, loc: &mut Loc) -> LexerResult where R: Read + Seek {}
macro_rules! lex_line_comment { ($this:ident, $reader:ident) => { {
    let mut buf: &mut [u8] = &mut [0;1];

    let c = $this.read($reader)?;
    $this.loc.add_pos();  // anyway add pos
    if c == b';' {
        loop {
            if let Ok(n) = $reader.read(&mut buf) {
                if n == 0 {
                    // end of line comment
                    $this.current = 0xFF;
                    return Ok(Token::empty($this.loc.clone()));
                }
                let com_c = buf[0];
                match com_c {
                    // end of line comment
                    b'\n' => { $this.loc.newline(); break; },
                    _ => { $this.loc.add_pos(); },
                }
            } else {
                return Err(LexError::io());  // rarely err
            }
        }
    } else {
        return Err(LexError::invalid_char(c, $this.loc.clone()));
    }

} }
}

impl Lexer {

pub fn lex_block_comment<R>(&mut self, reader: &mut R) -> Result<(), LexError>
    where R: Read + Seek {

    let mut com_c = self.read(reader)?;
    loop {
        match com_c {
            // maybe start of child block comment
            b'(' => {
                self.loc.add_pos();

                let com_c2 = self.read(reader)?;
                if com_c2 == b';' {
                    // start child block comment...
                    self.lex_block_comment(reader)?
                }
                com_c = com_c2;
                continue;
            }

            // maybe end of block comment
            b';' => {
                self.loc.add_pos();

                let com_c2 = self.read(reader)?;
                self.loc.add_pos();  // anyway add pos
                if com_c2 == b')' {
                    // end of block comment
                    return Ok(());
                }
                com_c = com_c2;
                continue;
            }

            // space (LF)
            b'\n' => {
                self.loc.newline();
            },

            // space (CR)
            b'\r' => {},

            // count byte as codepoint (not utf-8 bit pattern)
            0x00 ... 0x7F => self.loc.add_pos(),
            0xC2 ... 0xDF | 0xE0 ... 0xEF => {
                self.loc.add_pos();
                self.read(reader)?;
            },
            0xF0 ... 0xF7 => {
                self.loc.add_pos();
                self.read(reader)?;
                self.read(reader)?;                
            },

            // EOF
            0xFF => return Err(LexError::eof(&self.loc)),

            _ => { self.loc.add_pos(); },
        }

        com_c = self.read(reader)?;
    }
}


pub fn read(&mut self, reader: &mut (impl Read + Seek)) -> Result<u8, LexError> {
    let mut buf: &mut [u8] = &mut [0;1];    
    let n = reader.read(&mut buf)?;

    if n == 0 { return Ok(0xFF) }    
    Ok(buf[0])
}

}