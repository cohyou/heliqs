mod annot;
mod token;
mod cst;
mod indices;
mod instr;

pub use self::annot::*;
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Expr {
    pub instrs: Vec<Instr>
}

pub type Id = String;

// テキストフォーマット独自のstruct
// 要するにシンボルテーブル
#[derive(Debug, Default, Clone)]
pub struct Context {
    pub types: Vec<Option<Id>>,
    pub funcs: Vec<Option<Id>>,
    pub tables: Vec<Option<Id>>,
    pub mems: Vec<Option<Id>>,
    pub globals: Vec<Option<Id>>,
    pub locals: Vec<Option<Id>>,
    pub labels: Vec<Option<Id>>,
    pub typedefs: Vec<FuncType>, // typedefs functype*
}

impl Context {
    pub fn new() -> Context {
        Context::default()
    }
}

#[derive(Debug)]
pub struct Import {
    pub module_name: String,
    pub element_name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub func_type: TypeIndex, // type: typeuse
    pub locals: Vec<ValType>, // locals: vec(valtype)
    pub body: Expr, // body: expr
}

impl Func {
    pub fn new() -> Func {
        Func { func_type: TypeIndex::default(), locals: vec![], body: Expr { instrs: vec![] } }
    }
}

#[derive(Debug, Default)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug)]
pub enum ElemType {
    AnyFunc,
}

#[derive(Debug)]
pub struct TableType {
    pub limits: Limits,
    pub elem_type: ElemType,
}

#[derive(Debug, Default)]
pub struct MemType {
    pub limits: Limits,
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
pub struct GlobalType(pub Mutablity, pub ValType);

#[derive(Debug)]
pub enum ImportDesc {
    Func(TypeIndex),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct Table { pub table_type: TableType }

impl Table {
    pub fn new() -> Table {
        Table { table_type:
                TableType {
                    limits: Limits { min: 0, max: None },
                    elem_type: ElemType::AnyFunc
                }
        }
    }
}

#[derive(Debug, Default)]
pub struct Memory { pub memory_type: MemType }

impl Memory {
    pub fn new() -> Memory {
        Memory { memory_type: MemType { limits: Limits { min: 0, max: None } } }
    }
}

#[derive(Debug)]
pub struct Global {
    pub global_type: GlobalType,
    pub init: Expr,
}

impl Global {
    pub fn new() -> Global {
        Global {
            global_type: GlobalType::default(),
            init: Expr::default(),
        }
    }
}

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIndex),
    Table(TableIndex),
    Mem(MemIndex),
    Global(GlobalIndex),
}

#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Debug, Default, Clone)]
pub struct Start {
    pub func: FuncIndex,
}

#[derive(Debug, Default)]
pub struct Elem {
    pub table: TableIndex,
    pub offset: Expr,
    pub init: Vec<FuncIndex>,
}

type DataString = String;

#[derive(Debug, Default)]
pub struct Data {
    pub data: MemIndex,
    pub offset: Expr,
    pub init: DataString,
}

#[derive(Debug, Default)]
pub struct Module {
    pub id: Option<String>,
    pub types: Vec<FuncType>,
    pub imports: Vec<Import>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Memory>,
    pub globals: Vec<Global>,
    pub exports: Vec<Export>,
    pub start: Option<Start>,
    pub elems: Vec<Elem>,
    pub data: Vec<Data>,
}

impl Module {
    pub fn new() -> Module {
        Module::default()
    }
}