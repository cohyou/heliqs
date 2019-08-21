use instr::*;
use super::*;

macro_rules! instr_id {
    ($this:ident, $v:ident, $instr:ident, $indices:expr) => {{
        $this.consume()?;
        let local_id = $this.resolve_id(&$indices.clone())?;
        $v.push(Instr::$instr(local_id));
    }};
}

macro_rules! instr_local {
    ($this:ident, $v:ident, $instr:ident) => {{
        instr_id!($this, $v, $instr, $this.contexts[1].locals);
    }};
}

macro_rules! instr_global {
    ($this:ident, $v:ident, $instr:ident) => {{
        instr_id!($this, $v, $instr, $this.contexts[0].globals);
    }};
}

macro_rules! instr_func {
    ($this:ident, $v:ident, $instr:ident) => {{
        instr_id!($this, $v, $instr, $this.contexts[0].funcs);
    }};
}

macro_rules! instr_label {
    ($this:ident, $v:ident, $instr:ident) => {{
        instr_id!($this, $v, $instr, $this.contexts[1].labels);
    }};
}

macro_rules! instr_const {
    ($this:ident, $v:ident, $instr:ident, $tp:ident, $err:expr) => {{
        $this.consume()?;
        if let nm!(Number::Unsigned(n)) = $this.lookahead {
            $v.push(Instr::$instr(n as $tp));
            $this.consume()?;
        } else {
            return Err($this.err2($err));
        }
    }};
}

macro_rules! instr_memarg {
    ($this: ident, $v:ident, $align:expr) => {{
        $this.consume()?;
        let memarg = MemArg { align: $align, offset: 0 };
        $v.push(Instr::I32Load(memarg));        
    }};
}

impl<R> Parser<R> where R: Read + Seek {    
    pub(super) fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut instrs = vec![];

        loop {
            match &self.lookahead {
                // Control Instructions
                // instr!(Instr::Block(_, _) => ,
                // instr!(Instr::Loop(_, _) => ,
                // instr!(Instr::If(_, _, _) => ,
                // instr!(Instr::Br(_) => instr_label!(self, instrs, BrIf),
                // instr!(Instr::BrIf(_) => instr_label!(self, instrs, BrIf),
                // instr!(Instr::BrTable(_, _) => ,
                instr!(Instr::Call(_)) => instr_func!(self, instrs, Call),
                // instr!(Instr::CallIndirect(_)) => instr_func!(self, instrs, CallIndirect),

                // Variable Instructions
                instr!(Instr::LocalGet(_)) => instr_local!(self, instrs, LocalGet),
                instr!(Instr::LocalSet(_)) => instr_local!(self, instrs, LocalSet),
                instr!(Instr::LocalTee(_)) => instr_local!(self, instrs, LocalTee),
                instr!(Instr::GlobalGet(_)) => instr_global!(self, instrs, GlobalGet),
                instr!(Instr::GlobalSet(_)) => instr_global!(self, instrs, GlobalSet),

                // Memory Instructions
                instr!(Instr::I32Load(_)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::I64Load(_)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::F32Load(_)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::F64Load(_)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::I32Load8S(_)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::I32Load8U(_)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::I32Load16S(_)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::I32Load16U(_)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::I64Load8S(_)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::I64Load8U(_)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::I64Load16S(_)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::I64Load16U(_)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::I64Load32S(_)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::I64Load32U(_)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::I32Store(_)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::I64Store(_)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::F32Store(_)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::F64Store(_)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::I32Store8(_)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::I32Store16(_)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::I64Store8(_)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::I64Store16(_)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::I64Store32(_)) => instr_memarg!(self, instrs, 2),

                // Numeric Instructions
                instr!(Instr::I32Const(_)) => instr_const!(self, instrs, I32Const, u32, "i32.const"),
                instr!(Instr::I64Const(_)) => instr_const!(self, instrs, I64Const, u64, "i64.const"),
                // instr!(Instr::F32Const(_)) => {},
                // instr!(Instr::F64Const(_)) => {},

                instr!(instr) => {
                    instrs.push(instr.clone());
                    self.consume()?;
                },
                tk!(TokenKind::RightParen) => {
                    break;
                }
                _ => {},
            }
        }

        Ok(Expr(instrs))
    }
}
