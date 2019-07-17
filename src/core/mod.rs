use std::fmt::Debug;

mod token;
mod indices;
mod instr;

pub use self::token::*;
pub use self::indices::*;
pub use self::instr::*;

#[derive(PartialEq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}

macro_rules! expecting {
    ($s:ident, $p:pat, $r:expr, $m: expr) => {
        if let $p = $s {
            $r
        } else {
            panic!("{} {:?}", $m, $s);
        }        
    };
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
        expecting!(self, Tree::Leaf(Token::Symbol(s)), s.clone(), message)
    }

    pub fn expect_name(&self, message: &'static str) -> String {
        expecting!(self, Tree::Leaf(Token::Name(n)), n.clone(), message)
    }

    pub fn expect_valtype(&self, message: &'static str) -> ValType {
        expecting!(self, Tree::Leaf(Token::ValType(vt)), vt.clone(), message)
    }

    pub fn match_token(&self, token: Token) -> bool {
        if let Tree::Leaf(t) = self {
            t == &token
        } else {
            false
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

// Function types classify the signature of functions, 
// mapping a vector of parameters to a vector of results, written as follows.
// ということで、パラメータ列から結果列への写像、らしいです。
// ひとまず、vecのタプルとして持ちます。
pub type FuncType = (Vec<ValType>, Vec<ValType>);

// expr ::= instr* end
// expressionの長さはlimitationとして実装ごとに決定できる
// ひとまず、usizeにしておこう
#[derive(Debug, Clone)]
pub struct Expr {
    pub instrs: Vec<Instr>
}

// テキストフォーマット独自のstruct
// 要するにシンボルテーブル
#[derive(Debug)]
struct Context {
    typedefs: Vec<FuncType>, // typedefs functype*
}

#[derive(Debug, Default, Clone)]
pub struct TypeUse(pub TypeIndex); // 本来はContextが必要

impl TypeUse {
    pub fn type_index(&self) -> usize {
        // 現在は直接.0をとるだけだが、
        // Contextが付いた時に型の名前からもindexを取れるようにしたい
        self.0 as usize
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    pub func_type: TypeUse, // type: typeuse 
    pub locals: Vec<ValType>, // locals: vec(valtype)
    pub body: Expr, // body: expr
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
pub enum Mutablity {
    Const,
    Var,
}

impl Default for Mutablity {
    fn default() -> Self {
        Mutablity::Const
    }
}

#[derive(Debug, Default)]
pub struct GlobalType(Mutablity, ValType);

#[derive(Debug)]
pub enum ImportDesc {
    Func(TypeUse),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct Import {
    pub module_name: String,
    pub element_name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, Default, Clone)]
pub struct Start {
    pub func: FuncIndex,
}

#[derive(Debug)]
pub struct Module {
    pub id: Option<String>,
    pub types: Vec<FuncType>,
    pub imports: Vec<Import>,
    pub funcs: Vec<Func>,
    pub start: Option<Start>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            id: None, 
            types: vec![],
            imports: vec![],
            funcs: vec![],
            start: None,            
        }
    }
}