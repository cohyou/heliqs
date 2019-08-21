use instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub(super) fn parse_table(&mut self) -> Result<(), ParseError> {        
        // tabletype
        let table_type = self.parse_table_type()?;

        self.module.tables.push(Table(table_type));

        Ok(())
    }

    pub(super) fn parse_table_type(&mut self) -> Result<TableType, ParseError> {
        let mut table_type = TableType{ limits: Limits::default(), elem_type: ElemType::FuncRef };
        self.match_keyword(Keyword::Table)?;

        // table id
        parse_optional_id!(self, self.context().tables);

        // limits
        table_type.limits = self.parse_limits()?;

        // 'funcref'
        self.match_keyword(Keyword::FuncRef)?;

        self.match_rparen()?;
        Ok(table_type)
    }
}