use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_func(&mut self, module: &mut Module) -> Result<(), ParseError> {
        self.match_keyword(Keyword::Func)?;

        // func id
        parse_optional_id!(self, self.context().funcs);

        // typeuse
        // let typeidx = self.resolve_id(&self.contexts[0].types.clone())?;

        // locals

        // Expr

        module.funcs.push(Func(0, vec![], Expr(vec![])));

        self.match_rparen()?;

        Ok(())
    }

    // fn parse_functype(&mut self) -> Result<FuncType, ParseError> {
    //     let mut tp = FuncType::default();

    //     self.match_keyword(Keyword::Func)?;

    //     self.parse_signature(&mut tp)?;

    //     Ok(tp)
    // }
}