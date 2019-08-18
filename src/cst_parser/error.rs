 use lexer::{Token, TokenKind};
 
 #[derive(Debug)]
 pub enum CstParseError {
     Lex(LexError),
     NotMatch(Token, TokenKind),
 }

use lexer::LexError;
impl From<LexError> for CstParseError {
    fn from(e: LexError) -> Self { CstParseError::Lex(e) }
}
