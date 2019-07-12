use std::io::{Read, Seek};

use core::Module;
use token::Token;
use lexer::lex;

// module ::= '(' 'module' id^? (m: modulefield_I)^* ')' => 丸プラスm^*
// (if I = 丸プラスidc(modulefield)^* well-formed)

macro_rules! current_token {
    ($self: ident) => {
        $self.lookahead[$self.p]
    };
}

#[derive(Debug)]
pub struct Parser {
    pub module: Module,
    lookahead: [Token; 2],
    p: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            module: Module::new(),
            lookahead: [Token::Empty, Token::Empty],
            p: 0,
        }
    }

    fn consume(&mut self, reader: &mut (impl Read + Seek)) {
        if let Some(t) = lex(reader) {
            self.lookahead[self.p] = t;
            self.p = 1 - self.p;
        } else {
            println!("{:?}", "error");
        }
    }

    pub fn parse_module(&mut self, reader: &mut (impl Read + Seek)) {
        self.consume(reader);
        
        match current_token!(self) {
            Token::Module => self.parse_normal_module(reader),
            _ => self.parse_inline_module(reader),
        }
    }

    fn parse_normal_module(&mut self, reader: &mut (impl Read + Seek)) {        
        if let Some(Token::Name(n)) = lex(reader) {
            self.module.id = Some(n);
        }
        self.parse_module_fields(reader)
    }

    fn parse_inline_module(&self, _reader: &mut (impl Read + Seek)) {
        ;
    }

    // moduleの中身
    fn parse_module_fields(&self, reader: &mut (impl Read + Seek)) {
        self.parse_module_fields_recursive(reader)
    }
    // moduleの中身(再帰用)
    fn parse_module_fields_recursive(&self, reader: &mut (impl Read + Seek)) {
        self.parse_funcs(reader)
    }

    // funcのリスト
    fn parse_funcs(&self, reader: &mut (impl Read + Seek)) {
        self.parse_func(reader)
    }

    // func:    ( func <name>? <func_type> <local>* <instr>* )
    //          ( func <name>? ( export <string> ) <...> )                         ;; = (export <string> (func <N>)) (func <name>? <...>)
    //          ( func <name>? ( import <string> <string> ) <func_type>)           ;; = (import <name>? <string> <string> (func <func_type>))
    fn parse_func(&self, _reader: &mut (impl Read + Seek)) {
        ;
    }

    // fn parse_import(reader: &mut (impl Read + Seek)) {
    //     if let Some(Token::LeftParen) = lexer::lex(reader) {} else { reader.seek(SeekFrom::Current(-1)).unwrap(); return; }
    //     if let Some(Token::Import) = lexer::lex(reader) {} else { reader.seek(SeekFrom::Current(-2)).unwrap(); return; }
    //     if let Some(Token::RightParen) = lexer::lex(reader) {} else { reader.seek(SeekFrom::Current(-3)).unwrap(); return; }    
    // }
}



