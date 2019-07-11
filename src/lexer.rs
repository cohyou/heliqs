use std::io::Read;
use token::Token;

pub struct Lexer {
    reader: Box<dyn Read>,    
    current: u8,
}

impl Lexer {
    pub fn new(reader: impl Read + 'static) -> Lexer {
        Lexer {
            reader: Box::new(reader),            
            current: 0,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let reader = &mut self.reader;
        let mut bytes = reader.bytes().peekable();        
        // let mut current: u8 = 0;

        loop {
            if let Some(Ok(c)) = bytes.peek() {                
                println!("peeking: {:?}", c);
                match c {
                    b' ' => {
                        let c = bytes.next().unwrap().unwrap(); // consume                        
                        while c == b' ' {
                            if let Some(Ok(new_c)) = bytes.next() {
                                self.current = new_c;
                                if new_c != b' ' { break; }
                            } else {
                                return None;
                            }
                        }                        
                    },
                    b'(' => {
                        self.current = bytes.next().unwrap().unwrap(); // consume
                        // println!("( current: {:?}", self.current);
                        return Some(Token::LeftParen);
                    },
                    b')' => {
                        self.current = bytes.next().unwrap().unwrap(); // consume
                        // println!(") current: {:?}", self.current);
                        return Some(Token::RightParen);
                    },
                    _ => {
                        let mut token_bytes = vec![];
                        // self.current = bytes.next().unwrap().unwrap(); // consume                        
                        self.current = c.clone();
                        loop {
                            token_bytes.push(self.current);
                            if let Some(Ok(c)) = bytes.next() { // consume
                                match c {
                                    b' ' | b'(' | b')' => break,
                                    _ => {
                                        self.current = bytes.next().unwrap().unwrap();
                                        // self.current = b'x';
                                    },
                                }                                
                            } else {
                                break;
                            }
                            // self.current = bytes.next().unwrap().unwrap(); // consume
                            
                            // if let Some(Ok(c)) = bytes.next() { // consume
                            //     self.current = c;
                            //     match c {
                            //         b' ' | b'(' | b')' => break,
                            //         _ => {                                        
                            //             token_bytes.push(c);
                            //         },
                            //     }                                
                            // } else {
                            //     return None;
                            // }                            
                        }
                        return Some(Token::Name(String::from_utf8(token_bytes).unwrap()));
                    },
                }
            } else {
                // println!("current: {:?}", self.current);
                return None;
                // match self.current {
                //     b'(' => return Some(Token::LeftParen),
                //     b')' => return Some(Token::RightParen),
                //     _ => return None,
                // }
            }
        }
    }
}

#[test]
fn test_lexer() {
    use std::io::Cursor;
    let a = "(module (func))";
    let cur1 = Cursor::new(a);

    let mut lexer = Lexer::new(cur1);
    while let Some(t) = lexer.next() {
        println!("{:?}", t);
    }        
}