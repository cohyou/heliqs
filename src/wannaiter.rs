#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Module,
    Name(String),
}

// イテレータを自分で作りたい！
pub struct WannaIter {
    pub i: u16,
}

impl WannaIter {
    fn new() -> WannaIter {
        WannaIter { i: 0 }
    }
}

impl Iterator for WannaIter {
    type Item = Token;
    fn next(&mut self) -> std::option::Option<Self::Item> {
        self.i = self.i * 2;

        if self.i > 1000 {
            None
        } else {
            Some(Token::Name(self.i.to_string()))
        }    
    }
}