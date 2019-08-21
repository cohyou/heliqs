use std::fmt::Debug;
use super::Module;

impl Debug for Module {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   writeln!(f, "{{")?;
   if self.id.is_some() { writeln!(f, "  id: {:?}", self.id)?; }
   if self.types.len() > 0 {
      writeln!(f, "  types:")?;
      for tp in &self.types {
         writeln!(f, "    {:?}", tp)?;
      }
   }
   if self.imports.len() > 0 {
      writeln!(f, "  imports:")?;
      for import in &self.imports {
         writeln!(f, "    {:?}", import)?;
      }
   }
   if self.funcs.len() > 0 {
      writeln!(f, "  funcs:")?;
      for func in &self.funcs {
         writeln!(f, "    {:?}", func)?;
      }
   }
   if self.tables.len() > 0 { writeln!(f, "  tables: {:?}", self.tables)?; }
   if self.mems.len() > 0 { writeln!(f, "  mems: {:?}", self.mems)?; }
   if self.globals.len() > 0 {
      writeln!(f, "  globals:")?;
      for global in &self.globals {
         writeln!(f, "    {:?}", global)?;
      }
   }
   if self.exports.len() > 0 { writeln!(f, "  exports: {:?}", self.exports)?; }
   if self.start.is_some() { writeln!(f, "  start: {:?}", self.start)?; }
   if self.elems.len() > 0 { writeln!(f, "  elems: {:?}", self.elems)?; }
   if self.data.len() > 0 { writeln!(f, "  data: {:?}", self.data)?; }
   writeln!(f, "}}")        
}
}
