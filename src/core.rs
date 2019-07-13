use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Module,
    Import,
    Func,
    Symbol(String),
    Name(String), // $で始まる
    Empty,
}

#[derive(PartialEq)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}

pub type CST = Tree<Token>;

impl Debug for CST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Tree::Leaf(l) => {
                write!(f, "{:?}", l)
            },
            Tree::Node(v) => {
                write!(f, "{:?}", v)
            },
        }
    }
}


#[derive(Debug)]
pub struct Module {
    pub id: Option<String>,
    pub funcs: Vec<Func>,
}

#[derive(Debug)]
pub struct Func {

}

impl Module {
    pub fn new() -> Module {
        Module {
            id: None, 
            funcs: vec![]
        }
    }
}