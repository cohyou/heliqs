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
        instr_id!($this, $v, $instr, $this.contexts.last().unwrap().labels);
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
        $v.push(Instr::Load(ValType::I32, memarg));
    }};
}

macro_rules! instr_one_block {
    ($this:ident, $v:ident, $instr:ident) => {{
        $this.consume()?;

        // label id
        let mut new_label_context = $this.contexts.last().unwrap().clone();
        parse_optional_label_id!($this, new_label_context.labels);
        $this.contexts.push(new_label_context);

        $this.match_lparen()?;

        // resulttype
        let vt = $this.parse_result()?;

        // expr
        let expr = $this.parse_expr()?;

        $this.match_keyword(Keyword::End)?;

        // label id(repeated)
        $this.check_label_id()?;

        p!($this.contexts.last());
        $this.contexts.pop();

        $v.push(Instr::$instr(vec![vt], expr));
    }};
}

impl<R> Parser<R> where R: Read + Seek {

    pub(super) fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut instrs = vec![];

        loop {
            match &self.lookahead {
                // Control Instructions
                instr!(Instr::Block(_, _)) => instr_one_block!(self, instrs, Block),
                instr!(Instr::Loop(_, _)) => instr_one_block!(self, instrs, Loop),
                instr!(Instr::If(_, _, _)) => self.parse_if(&mut instrs)?,
                instr!(Instr::Br(_)) => instr_label!(self, instrs, BrIf),
                instr!(Instr::BrIf(_)) => instr_label!(self, instrs, BrIf),
                instr!(Instr::BrTable(_, _)) => self.parse_br_table(&mut instrs)?,
                instr!(Instr::Call(_)) => instr_func!(self, instrs, Call),
                instr!(Instr::CallIndirect(_)) => self.parse_call_indirect(&mut instrs)?,

                // Variable Instructions
                instr!(Instr::LocalGet(_)) => instr_local!(self, instrs, LocalGet),
                instr!(Instr::LocalSet(_)) => instr_local!(self, instrs, LocalSet),
                instr!(Instr::LocalTee(_)) => instr_local!(self, instrs, LocalTee),
                instr!(Instr::GlobalGet(_)) => instr_global!(self, instrs, GlobalGet),
                instr!(Instr::GlobalSet(_)) => instr_global!(self, instrs, GlobalSet),

                // Memory Instructions
                instr!(Instr::ILoad8(_, _, _)) => instr_memarg!(self, instrs, 0),
                instr!(Instr::IStore8(_, _)) => instr_memarg!(self, instrs, 0),

                instr!(Instr::ILoad16(_, _, _)) => instr_memarg!(self, instrs, 1),
                instr!(Instr::IStore16(_, _)) => instr_memarg!(self, instrs, 1),

                instr!(Instr::Load(ValType::I32, _)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::Load(ValType::F32, _)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::I64Load32(_, _)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::Store(ValType::I32, _)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::Store(ValType::F32, _)) => instr_memarg!(self, instrs, 2),
                instr!(Instr::I64Store32(_)) => instr_memarg!(self, instrs, 2),

                instr!(Instr::Load(ValType::I64, _)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::Load(ValType::F64, _)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::Store(ValType::I64, _)) => instr_memarg!(self, instrs, 3),
                instr!(Instr::Store(ValType::F64, _)) => instr_memarg!(self, instrs, 3),

                // Numeric Instructions
                instr!(Instr::I32Const(_)) => instr_const!(self, instrs, I32Const, u32, "i32.const"),
                instr!(Instr::I64Const(_)) => instr_const!(self, instrs, I64Const, u64, "i64.const"),
                // instr!(Instr::F32Const(_)) => {},
                // instr!(Instr::F64Const(_)) => {},

                instr!(instr) => {
                    instrs.push(instr.clone());
                    self.consume()?;
                },

                tk!(TokenKind::RightParen) | kw!(Keyword::Else) | kw!(Keyword::End) => {
                    break;
                }
                _ => {},
            }
        }

        Ok(Expr(instrs))
    }

    fn parse_call_indirect(&mut self, instrs: &mut Vec<Instr>) -> Result<(), ParseError> {
        self.consume()?;

        let mut _ft = FuncType::default();

        // add local context(for check)
        self.contexts.push(Context::default());

        let typeidx = self.parse_typeuse(&mut _ft.0, &mut _ft.1)?;
        self.check_typeuse(typeidx, _ft)?;    

        // check params context (must not include string id)
        if self.contexts[2].locals.iter().any(|x| x.is_some()) {
            p!(self.contexts[2].locals);
            Err(self.err2("call_indirect: params context (must be empty)"))
        } else {
            instrs.push(Instr::CallIndirect(typeidx));

            la!(self);p!(self.contexts[2]);
            self.contexts.pop();

            Ok(())        
        }        
    }

    fn parse_if(&mut self, instrs: &mut Vec<Instr>) -> Result<(), ParseError> {
        self.consume()?;

        // label id
        let mut new_label_context = self.contexts.last().unwrap().clone();
        parse_optional_label_id!(self, new_label_context.labels);
        self.contexts.push(new_label_context);

        self.match_lparen()?;

        // resulttype
        let vt = self.parse_result()?;

        // expr1
        let expr1 = self.parse_expr()?;

        self.match_keyword(Keyword::Else)?;

        // check label id(after else)
        self.check_label_id()?;

        // expr2
        let expr2 = self.parse_expr()?;

        self.match_keyword(Keyword::End)?;

        // check label id(after end)
        self.check_label_id()?;
    
        p!(self.contexts.last());
        self.contexts.pop();

        instrs.push(Instr::If(vec![vt], expr1, expr2));

        Ok(())
    }

    fn parse_br_table(&mut self, instrs: &mut Vec<Instr>) -> Result<(), ParseError> {
        self.consume()?;

        let mut labelindices = vec![];

        loop {
            match &self.lookahead {
                tk!(TokenKind::Id(_)) => {
                    let local_id = self.resolve_id(&self.contexts.last().unwrap().clone().labels)?;
                    labelindices.push(local_id);
                },
                nm!(Number::Unsigned(n)) => {
                    labelindices.push(*n as u32);
                    self.consume()?;
                },
                _ => break,
            }
        }

        if let Some(labelidx) = labelindices.pop() {
            instrs.push(Instr::BrTable(labelindices, labelidx));
        } else {
            return Err(self.err2("br_table"));
        }

        Ok(())
    }

    fn check_label_id(&mut self) -> Result<(), ParseError> {
        if let tk!(TokenKind::Id(s)) = &self.lookahead {

            if let Some(label_s) = &self.contexts.last().unwrap().labels.last().unwrap() {
                if s != label_s {
                    return Err(self.err2("invalid label of block end"));
                }
            } else {
                return Err(self.err2("invalid label of block end"));
            }
            self.consume()?;
        }

        Ok(())
    }
}
