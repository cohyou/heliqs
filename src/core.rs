use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Module,
    Import,
    Type,
    Func,
    Param,
    FuncResult,
    ValType(ValType),
    Symbol(String),
    Name(String), // $で始まる
    End,
    Empty,
}

#[derive(PartialEq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>),
}

pub type CST = Tree<Token>;

impl CST {
    pub fn unwrap(&self) -> Vec<Tree<Token>> {
        if let Tree::Node(v) = self {
            v.to_vec()
        } else {
            panic!("あかーん");
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


#[derive(Debug)]
pub struct Module {
    pub id: Option<String>,
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
}

type TypeIndex = u32;
type FuncIndex = u32;

// valtype ::= i32 | i64 | f32 | f64
#[derive(Debug, PartialEq, Clone)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

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

#[derive(Debug)]
struct TypeUse(TypeIndex); // 本来はContextが必要

#[derive(Debug)]
pub struct Func {
    func_type: Option<TypeUse>, // type: typeuse 
    locals: Vec<ValType>, // locals: vec(valtype)
    body: Expr, // body: expr
}

impl Func {
    pub fn new() -> Func {
        Func { func_type: None, locals: vec![], body: Expr { instrs: vec![] } }
    }
}

impl Module {
    pub fn new() -> Module {
        Module {
            id: None, 
            types: vec![],
            funcs: vec![],
        }
    }
}