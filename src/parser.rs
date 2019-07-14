use std::io::{Read, Seek};

// use core::Module;
use core::{Token, Tree, CST};
use lexer::lex;

// module ::= '(' 'module' id^? (m: modulefield_I)^* ')' => 丸プラスm^*
// (if I = 丸プラスidc(modulefield)^* well-formed)

// macro_rules! current_token {
//     ($self: ident) => {
//         $self.lookahead[$self.p]
//     };
// }

#[derive(Debug)]
pub struct Parser {
    lookahead: Token,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {            
            lookahead: Token::Empty,
        }
    }

    fn consume(&mut self, reader: &mut (impl Read + Seek)) {
        if let Some(t) = lex(reader) {
            self.lookahead = t;
        } else {
            // println!("consume error: {:?}", self.lookahead);
        }
    }

    pub fn parse(&mut self, reader: &mut (impl Read + Seek)) -> Option<CST> {
        if let Some(t) = lex(reader) {
            self.lookahead = t;
            self.parse_elements(reader)
        } else {
            // 本当はいきなりEOFの場合は受理しそう
            // (module) と同じになるはず
            None
        }
    }

    // 空白で区切られたexpressionを
    fn parse_elements(&mut self, reader: &mut (impl Read + Seek)) -> Option<CST> {
        let mut result: Vec<CST> = vec![];

        loop {
            if let Some(tree) = self.parse_element(reader) {
                // println!("parse_elements tree: {:?}", tree);
                result.push(tree);

                if self.lookahead == Token::RightParen {                    
                    // 終わりなので、結果を返す
                    // println!("parse_elements result: {:?}", result);

                    return Some(Tree::Node(result));
                }
            } else {
                return None;
            }
        }
    }

    fn parse_element(&mut self, reader: &mut (impl Read + Seek)) -> Option<CST> {
        match self.lookahead {
            Token::LeftParen => {
                // リストの始まり
                self.parse_list(reader)
            },
            Token::Module | 
            
            Token::Type | 
            Token::Func | 
            Token::Import | 
            Token::Start | 

            Token::Local | 
            Token::Param | 
            Token::FuncResult | 

            Token::Call |

            Token::ValType(_) |
            Token::Name(_) | 
            Token::Text(_) |             
            Token::Symbol(_) => {                
                let r = Some(Tree::Leaf(self.lookahead.clone()));
                self.consume(reader);
                r
            }
            _ => None,
        }
    }

    fn match_token(&mut self, reader: &mut (impl Read + Seek), t: Token) {
        if self.lookahead == t {
            self.consume(reader)
        } else {
            println!("match_token error: {:?}", self.lookahead);
        }
    }

    fn parse_list(&mut self, reader: &mut (impl Read + Seek)) -> Option<CST> {
        self.match_token(reader, Token::LeftParen);
        let r = self.parse_elements(reader);
        self.match_token(reader, Token::RightParen);
        r
    }

    // fn parse_module(&mut self, reader: &mut (impl Read + Seek)) {
    //     self.consume(reader);
        
    //     match current_token!(self) {
    //         Token::Module => self.parse_normal_module(reader),
    //         _ => self.parse_inline_module(reader),
    //     }
    // }

    // fn parse_normal_module(&mut self, reader: &mut (impl Read + Seek)) {        
    //     if let Some(Token::Name(n)) = lex(reader) {
    //         self.module.id = Some(n);
    //     }
    //     self.parse_module_fields(reader)
    // }

    // fn parse_inline_module(&self, _reader: &mut (impl Read + Seek)) {
    //     ;
    // }

    // // moduleの中身
    // fn parse_module_fields(&self, reader: &mut (impl Read + Seek)) {
    //     self.parse_module_fields_recursive(reader)
    // }
    // // moduleの中身(再帰用)
    // fn parse_module_fields_recursive(&self, reader: &mut (impl Read + Seek)) {
    //     self.parse_funcs(reader)
    // }

    // // funcのリスト
    // fn parse_funcs(&self, reader: &mut (impl Read + Seek)) {
    //     self.parse_func(reader)
    // }

    // func:    ( func <name>? <func_type> <local>* <instr>* )
    //          ( func <name>? ( export <string> ) <...> )                         ;; = (export <string> (func <N>)) (func <name>? <...>)
    //          ( func <name>? ( import <string> <string> ) <func_type>)           ;; = (import <name>? <string> <string> (func <func_type>))
    // fn parse_func(&self, _reader: &mut (impl Read + Seek)) {
    //     ;
    // }

    // fn parse_import(reader: &mut (impl Read + Seek)) {
    //     if let Some(Token::LeftParen) = lexer::lex(reader) {} else { reader.seek(SeekFrom::Current(-1)).unwrap(); return; }
    //     if let Some(Token::Import) = lexer::lex(reader) {} else { reader.seek(SeekFrom::Current(-2)).unwrap(); return; }
    //     if let Some(Token::RightParen) = lexer::lex(reader) {} else { reader.seek(SeekFrom::Current(-3)).unwrap(); return; }    
    // }
}



