use instr::FuncType;

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

pub type Id = String;

pub type TypeIndex = u32;
pub type FuncIndex = u32;
pub type TableIndex = u32;
pub type MemIndex = u32;
pub type GlobalIndex = u32;

pub type LocalIndex = u32;
pub type LabelIndex = u32;