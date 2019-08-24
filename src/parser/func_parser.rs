use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_func(&mut self) -> Result<(), ParseError> {
        let mut func = Func::default();

        self.match_keyword(Keyword::Func)?;

        // func id
        parse_optional_id!(self, self.contexts[0].funcs);

        // add local context
        self.contexts.push(Context::default());

        // typeuse
        let mut _ft = FuncType::default();
        func.0 = self.parse_typeuse(&mut _ft.0, &mut _ft.1)?;

        self.check_typeuse(func.0, _ft)?;

        let typedef = &self.contexts[0].typedefs[func.0 as usize];
        func.1.extend(typedef.0.clone());

        // locals
        parse_field!(self, Local, 
        if let Ok(local_vt) = self.parse_local() {
            func.1.push(local_vt);
        });

        // Expr
        func.2 = self.parse_expr()?;

        self.module.funcs.push(func);

        // la!(self);p!(self.contexts[1]);
        self.contexts.pop();
        self.match_rparen()?;

        Ok(())
    }

    fn parse_local(&mut self) -> Result<ValType, ParseError> {

        self.match_keyword(Keyword::Local)?;

        // local id
        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            let new_s = s.clone();
            self.contexts[1].locals.push(Some(new_s));
            self.consume()?;
        } else {
            self.contexts[1].locals.push(None);
        }

        let vt = self.parse_valtype()?;

        self.match_rparen()?;

        Ok(vt)
    }
}