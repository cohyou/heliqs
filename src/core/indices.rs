pub type TypeIndex = u32;
pub type FuncIndex = u32;

struct Context {
    types: Vec<Option<String>>,
    funcs: Vec<Option<String>>,
    tables: Vec<Option<String>>,
    mems: Vec<Option<String>>,
    globals: Vec<Option<String>>,
    locals: Vec<Option<String>>,
    labels: Vec<Option<String>>,
    typedefs: Vec<Option<String>>,
}