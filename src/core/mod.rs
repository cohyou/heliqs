mod token;
mod cst;
mod indices;
mod instr;

pub use self::token::*;
pub use self::cst::*;
pub use self::indices::*;
pub use self::instr::*;

// Function types classify the signature of functions, 
// mapping a vector of parameters to a vector of results, written as follows.
// ということで、パラメータ列から結果列への写像、らしいです。
// ひとまず、vecのタプルとして持ちます。
pub type FuncType = (Vec<ValType>, Vec<ValType>);

pub type ResultType = Vec<ValType>;

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

#[derive(Debug, Default)]
pub struct Memory { memory_type: MemType }

#[derive(Debug, Default, Clone)]
pub struct Start {
    pub func: FuncIndex,
}

#[derive(Debug)]
pub struct Import {
    pub module_name: String,
    pub element_name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, Default)]
pub struct Module {
    pub id: Option<String>,
    pub types: Vec<FuncType>,    
    pub funcs: Vec<Func>,
    pub mems: Vec<Memory>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
}

impl Module {
    pub fn new() -> Module {
        Module::default()
    }
}