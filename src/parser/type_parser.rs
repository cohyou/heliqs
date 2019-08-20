use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_type(&mut self, module: &mut Module) -> Result<(), ParseError> {

        self.match_keyword(Keyword::Type)?;

        // type id
        parse_optional_id!(self, self.context().types);

        // functype
        self.match_lparen()?;
        let functype = self.parse_functype()?;

        module.types.push(functype.clone());
        self.contexts[0].typedefs.push(functype);

        self.match_rparen()?;

        Ok(())
    }

    fn parse_functype(&mut self) -> Result<FuncType, ParseError> {
        let mut functype = FuncType::default();

        self.match_keyword(Keyword::Func)?;
lla!(10, self);        
        if !self.is_rparen()? {
            self.parse_signature(&mut functype.0, &mut functype.1)?;
        }        

        self.match_rparen()?;

        Ok(functype)
    }

}