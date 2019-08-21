use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub(super) fn parse_global(&mut self) -> Result<(), ParseError> {
        let global_type = self.parse_global_type()?;

        let expr = self.parse_expr()?;
        
        self.module.globals.push(Global(global_type, expr));

        self.match_rparen()?;

        Ok(())
    }

    pub(super) fn parse_global_type(&mut self) -> Result<GlobalType, ParseError> {        
        self.match_keyword(Keyword::Global)?;

        // global id
        parse_optional_id!(self, self.contexts[0].globals);

        // mutablity
        let mutablity = Mutablity::Const;

        // valtype
        let vt = if self.is_lparen()? {
            self.match_lparen()?;
            self.match_keyword(Keyword::Mutable)?;
            let vt = self.parse_valtype()?;
            self.match_rparen()?;
            vt 
        } else {
            self.parse_valtype()?
        };

        let global_type = GlobalType(mutablity, vt);

        Ok(global_type)
    }
}