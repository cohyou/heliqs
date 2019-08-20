mod impls;

use context::*;
use instr::{FuncType, Expr, ValType};

pub use self::impls::*;

#[derive(Default)]
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

#[derive(Debug)]
pub struct Import (pub Name, pub Name, pub ImportDesc);

#[derive(Debug, Clone, PartialEq)]
pub struct Func (
    pub TypeIndex,  // type: typeuse
    pub Vec<ValType>,  // locals: vec(valtype)
    pub Expr,  // body: expr
);

#[derive(Debug)]
pub struct Table(pub TableType);

#[derive(Debug, Default)]
pub struct Memory(pub MemType);

#[derive(Debug)]
pub struct Global(pub GlobalType, pub Expr);

#[derive(Debug)]
pub struct Export (pub String, pub ExportDesc);

#[derive(Debug, Default, Clone)]
pub struct Start (pub FuncIndex);

#[derive(Debug, Default)]
pub struct Elem {
    pub table: TableIndex,
    pub offset: Expr,
    pub init: Vec<FuncIndex>,
}

#[derive(Debug, Default)]
pub struct Data {
    pub data: MemIndex,
    pub offset: Expr,
    pub init: DataString,
}

#[derive(Debug)]
pub enum ImportDesc {
    Func(TypeIndex),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct TableType {
    pub limits: Limits,
    pub elem_type: ElemType,
}

#[derive(Debug, Default)]
pub struct MemType(pub Limits);

#[derive(Debug, Default)]
pub struct GlobalType(pub Mutablity, pub ValType);

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIndex),
    Table(TableIndex),
    Mem(MemIndex),
    Global(GlobalIndex),
}

#[derive(Debug, Default)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug)]
pub enum ElemType { FuncRef, }

#[derive(Debug)]
pub enum Mutablity { Const, Var, }

pub type Name = String;
pub type DataString = String;


impl Default for Mutablity { fn default() -> Self { Mutablity::Const } }
impl Default for ValType { fn default() -> Self { ValType::I32 } }

