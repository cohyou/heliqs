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