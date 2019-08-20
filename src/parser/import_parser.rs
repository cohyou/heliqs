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
        let table_type = self.parse_table_type()?;
        Ok(ImportDesc::Table(table_type))
    }

    fn parse_import_desc_memory(&mut self) -> Result<ImportDesc, ParseError> {        
        let mem_type = self.parse_memory_type()?;
        Ok(ImportDesc::Mem(mem_type))
    }

    fn parse_import_desc_global(&mut self) -> Result<ImportDesc, ParseError> {        
        let global_type = self.parse_global_type()?;
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
