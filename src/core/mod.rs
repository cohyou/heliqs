pub mod token;

use std::fmt::Debug;
pub use self::token::*;

#[derive(PartialEq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}

pub type CST = Tree<Token>;

impl CST {
    pub fn unwrap_node(&self) -> Vec<Tree<Token>> {
        if let Tree::Node(v) = self { v.to_vec() } else { panic!("あかーん"); }
    }

    pub fn expect_node(&self, message: &'static str) -> Vec<Tree<Token>> {
        if let Tree::Node(v) = self { v.to_vec() } else { panic!(message); }
    }

    pub fn expect_text(&self, message: &'static str) -> String {
        if let Tree::Leaf(Token::Text(s)) = self { s.clone() } else { panic!(message); }
    }

    pub fn expect_leaf(&self, message: &'static str) -> Token {
        if let Tree::Leaf(t) = self { t.clone() } else { panic!(message); }        
    }

    pub fn expect_symbol(&self, message: &'static str) -> String {
        if let Tree::Leaf(Token::Symbol(s)) = self { s.clone() } else {
            let s = format!("{} {:?}", message, self);
            panic!(s);
        }
    }
}

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


pub type TypeIndex = u32;
type FuncIndex = u32;


// Function types classify the signature of functions, 
// mapping a vector of parameters to a vector of results, written as follows.
// ということで、パラメータ列から結果列への写像、らしいです。
// ひとまず、vecのタプルとして持ちます。
pub type FuncType = (Vec<ValType>, Vec<ValType>);

#[derive(Debug)]
enum Instr {
    Call(FuncIndex),
}

// expr ::= instr* end
// expressionの長さはlimitationとして実装ごとに決定できる
// ひとまず、usizeにしておこう
#[derive(Debug)]
struct Expr {
    instrs: Vec<Instr>
}

// テキストフォーマット独自のstruct
// 要するにシンボルテーブル
#[derive(Debug)]
struct Context {
    typedefs: Vec<FuncType>, // typedefs functype*
}

#[derive(Debug, Default)]
pub struct TypeUse(pub TypeIndex); // 本来はContextが必要

#[derive(Debug)]
pub struct Func {
    pub func_type: TypeUse, // type: typeuse 
    pub locals: Vec<ValType>, // locals: vec(valtype)
    body: Expr, // body: expr
}

impl Func {
    pub fn new() -> Func {
        Func { func_type: TypeUse::default(), locals: vec![], body: Expr { instrs: vec![] } }
    }
}

#[derive(Debug, Default)]
struct Limits {
    min: u32,
    max: Option<u32>,
}

#[derive(Debug, Default)]
struct ElemType;

#[derive(Debug, Default)]
pub struct TableType {
    limits: Limits,
    elem_type: ElemType,
}

#[derive(Debug, Default)]
pub struct MemType {
    limits: Limits,
}

#[derive(Debug)]
enum MutablityType {
    Const,
    Var,
}

impl Default for MutablityType {
    fn default() -> Self {
        MutablityType::Const
    }
}

#[derive(Debug, Default)]
pub struct GlobalType(MutablityType, ValType);

#[derive(Debug, Default)]
pub struct ImportDesc {
    pub func: TypeUse,
    pub table: TableType,
    pub mem: MemType,
    pub global: GlobalType,
}

#[derive(Debug, Default)]
pub struct Import {
    pub module_name: String,
    pub element_name: String,
    pub desc: ImportDesc,
}

#[derive(Debug)]
pub struct Module {
    pub id: Option<String>,
    pub types: Vec<FuncType>,
    pub imports: Vec<Import>,
    pub funcs: Vec<Func>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            id: None, 
            types: vec![],
            imports: vec![],
            funcs: vec![],
        }
    }
}