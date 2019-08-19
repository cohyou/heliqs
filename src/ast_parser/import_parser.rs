use super::*;

impl<'a> AstParser<'a> {

pub fn parse_import<Iter>(&mut self, module: &mut Module, iter: &mut Iter) -> Result<(), AstParseError>
    where Iter: Iterator<Item=(&'a Cst, usize)> {

    self.match_keyword(iter, Keyword::Import)?;
    
    Err(self.err())
}

}