use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub(super) fn parse_import(&mut self, module: &mut Module) -> Result<(), ParseError> {
        self.match_keyword(Keyword::Import)?;

        // module
        let import_module = self.parse_name()?;

        // name
        let import_name = self.parse_name()?;

        self.match_lparen()?;

        // import desc
        let import_desc = self.parse_import_desc()?;

        module.imports.push(Import(import_module, import_name, import_desc));
        self.match_rparen()?;
        Ok(())
    }

    fn parse_import_desc(&mut self) -> Result<ImportDesc, ParseError> {
        match self.lookahead {
            kw!(Keyword::Func) => self.parse_import_desc_func(),
            kw!(Keyword::Table) => self.parse_import_desc_table(),
            kw!(Keyword::Memory) => self.parse_import_desc_memory(),
            kw!(Keyword::Global) => self.parse_import_desc_global(),
            _ => Err(self.err())
        }
    }

    fn parse_import_desc_func(&mut self) -> Result<ImportDesc, ParseError> {        
        self.match_keyword(Keyword::Func)?;

        // func id
        parse_optional_id!(self, self.context().funcs);

        // typeuse
        let typeidx = self.parse_typeuse()?;

        Ok(ImportDesc::Func(typeidx))
    }

    fn parse_import_desc_table(&mut self) -> Result<ImportDesc, ParseError> {
        let mut table_type = TableType{ limits: Limits{min: 0, max: None}, elem_type: ElemType::FuncRef };
        self.match_keyword(Keyword::Table)?;

        // table id
        parse_optional_id!(self, self.context().tables);

        // limits
        table_type.limits = self.parse_limits()?;

        // 'funcref'
        self.match_keyword(Keyword::FuncRef)?;

        self.match_rparen()?;
        Ok(ImportDesc::Table(table_type))
    }

    fn parse_import_desc_memory(&mut self) -> Result<ImportDesc, ParseError> {        
        self.match_keyword(Keyword::Memory)?;

        // mem id
        parse_optional_id!(self, self.context().mems);

        let limits = self.parse_limits()?;

        self.match_rparen()?;

        Ok(ImportDesc::Mem(MemType(limits)))
    }

    fn parse_limits(&mut self) -> Result<Limits, ParseError> {
        let mut limits = Limits::default();

        // min
        limits.min = self.parse_num::<u32>()?;

        // max(optional)
        if let nm!(Number::Unsigned(_)) = &self.lookahead {            
            limits.max = Some(self.parse_num::<u32>()?);
        }        

        Ok(limits)
    }

    fn parse_import_desc_global(&mut self) -> Result<ImportDesc, ParseError> {        
        self.match_keyword(Keyword::Global)?;

        // global id
        parse_optional_id!(self, self.context().globals);

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

        self.match_rparen()?;

        Ok(ImportDesc::Global(global_type))
    }
}

// fn perse_local(cst: &Cst) -> Option<(Option<&Id>, &ValType)> {
//     cst.list().and_then(|v| {
//         let mut v_iter = v.iter();

//         // 'local'
//         v_iter.next();

//         // id
//         let mut v_iter_peekable = v_iter.peekable();
//         let id = make_optional_id!(v_iter_peekable);

//         // ValType
//         v_iter_peekable.next()
//         .and_then(|cst| cst.valtype())
//         .map(|vt| (id, vt))
//     })
// }
