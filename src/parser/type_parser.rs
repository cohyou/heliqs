use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_type(&mut self, module: &mut Module) -> Result<(), ParseError> {

        self.match_keyword(Keyword::Type)?;

        // type id
        parse_optional_id!(self, self.context().types);

        // functype
        self.match_lparen()?;
        let tp = self.parse_functype()?;

        module.types.push(tp.clone());
        self.contexts[0].typedefs.push(tp);

        self.match_rparen()?;

        Ok(())
    }

    fn parse_functype(&mut self) -> Result<FuncType, ParseError> {
        let mut tp = FuncType::default();

        self.match_keyword(Keyword::Func)?;

        self.parse_signature(&mut tp)?;

        Ok(tp)
    }

}