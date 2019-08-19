mod cst;
mod error;

use std::io::{Read, Seek};

use annot::Loc;
use lexer::{Lexer, Token, TokenKind};

pub use self::cst::*;
pub use self::error::*;


#[derive(PartialEq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}

pub type Cst = Tree<Token>;

#[derive(Debug)]
pub struct CstParser {
    lexer: Lexer,
    lookahead: Token,
}

impl CstParser {
    pub fn new(reader: &mut (impl Read + Seek)) -> Self {
        Self {
            lexer: Lexer::new(reader),
            lookahead: Token::empty(Loc::default()),
        }
    }

    pub fn parse(&mut self, reader: &mut (impl Read + Seek)) -> Result<Cst, CstParseError> {
        self.lookahead = self.lexer.lex_token(reader)?;
        self.parse_elements(reader)
        // let node = self.parse_elements(reader)?;
        // if let Tree::Node(mut v) = node {
        //     v.push(Tree::Leaf(Token::right_paren(Loc::default())));
        //     Ok(Tree::Node(v))
        // } else {
        //     unreachable!();
        // }
    }

    // 空白で区切られたexpressionを
    fn parse_elements(&mut self, reader: &mut (impl Read + Seek)) -> Result<Cst, CstParseError> {
        let mut result: Vec<Cst> = vec![];

        if !self.is_closing() {
            loop {
                let tree = self.parse_element(reader)?;
                // println!("get element: {:?}", tree);

                result.push(tree);

                // 終わりなら、結果を返す
                if self.is_closing() {
                    result.push(Tree::Leaf(self.lookahead.clone()));
                    break;
                }
            }
        }

        // println!("parse_elements result: {:?}", result);
        Ok(Tree::Node(result))
    }

    fn is_closing(&self) -> bool {
        self.lookahead.value == TokenKind::RightParen ||
        self.lookahead.value == TokenKind::Empty
    }

    fn parse_element(&mut self, reader: &mut (impl Read + Seek)) -> Result<Cst, CstParseError> {
        match self.lookahead.value {
            TokenKind::LeftParen => {
                // リストの始まり
                self.parse_list(reader)
            },
            _ => {
                let r = Tree::Leaf(self.lookahead.clone());
                self.consume(reader)?;
                Ok(r)
            },
        }
    }

    fn match_token(&mut self, reader: &mut (impl Read + Seek), t: TokenKind) -> Result<(), CstParseError> {
        if self.lookahead.value == t {
            self.consume(reader)
        } else {
            Err(CstParseError::NotMatch(self.lookahead.clone(), t))
        }
    }

    fn parse_list(&mut self, reader: &mut (impl Read + Seek)) -> Result<Cst, CstParseError> {
        self.match_token(reader, TokenKind::LeftParen)?;
        let r = self.parse_elements(reader);
        self.match_token(reader, TokenKind::RightParen)?;
        r
    }

    fn consume(&mut self, reader: &mut (impl Read + Seek)) -> Result<(), CstParseError> {
        let lookahead = self.lexer.lex_token(reader)?;
        // println!("lookahead: {:?}", lookahead);
        self.lookahead = lookahead;
        Ok(())
    }
}