use std::str::FromStr;
use std::convert::TryFrom;
use core::*;
use super::util::*;
use super::func::*;

macro_rules! tk { ($kind:pat) => { Tree::Leaf(Annot{value: $kind, ..}) } }

macro_rules! make_const {
    ($v:ident, $t:ident, $ret:expr) => {{
        $v.next()
        .and_then(|cst| cst.symbol())
        .and_then(|num_str|
            num_str.parse::<$t>()
            .map(|num| $ret(num))
            .ok())
    }};
}

pub fn make_instrs<'a>(v: &mut (impl Iterator<Item=&'a CST>), context: &Context) -> Option<Vec<Instr>> {
    let mut instrs = vec![];
    let mut v = v.peekable();
    let mut instr_elem = v.next();
    loop {
        println!("instr_elem: {:?}", instr_elem);
        if let Some(tk!(token)) = instr_elem {
            if let Some(instr) = make_block_instr(token.clone(), &mut v, context) {
                instrs.push(instr);
            }
            if let Some(instr) = make_plain_instr(token.clone(), &mut v, context) {
                instrs.push(instr);
            }
        }
        if v.peek().is_none() { break; }
        instr_elem = v.next();
    }

    Some(instrs)
}

fn make_block_instr<'a>(token: TokenKind, v: &mut (impl Iterator<Item=&'a CST>), context: &Context) -> Option<Instr> {
    match token {
        // Control Instructions
        TokenKind::Block => Some(Instr::Block(vec![], Expr{ instrs: vec! [] })),
        TokenKind::Loop => Some(Instr::Loop(vec![], Expr{ instrs: vec! [] })),
        TokenKind::If => Some(Instr::If(vec![], Expr{ instrs: vec! [] }, Expr{ instrs: vec! [] })),
        _ => None,
    }
}

fn make_br_table<'a, Iter, T>(v: &mut Iter, indices: &Vec<Option<Id>>) -> Option<Instr>
    where Iter: Iterator<Item=&'a CST>,
          T: FromStr + TryFrom<usize> {
    // labelidxとして認識できる限りは取り続ける
    let mut lidxs = vec![];
    let mut v_peekable = v.peekable();

    loop {
        if let Some(item) = v_peekable.peek() {
            println!("item: {:?}", item);

            if let Some(idx) = v_peekable.peek()
            .and_then(|cst| cst.token())
            .and_then(|token| make_idx::<LabelIndex>(token, indices).ok()) {
                lidxs.push(idx);
                v_peekable.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if let Some(l) = lidxs.pop() {
        Some(Instr::BrTable(lidxs, l))
    } else {
        panic!("最低でも1つlabelidxが必要");
    }
}

fn make_plain_instr<'a, Iter>(token: TokenKind, v: &mut Iter, context: &Context) -> Option<Instr>
    where Iter: Iterator<Item=&'a CST> {

    match token {
        // Control Instructions
        TokenKind::Unreachable => Some(Instr::Unreachable),
        TokenKind::Nop => Some(Instr::Nop),
        TokenKind::Br => get_index(v, &context.labels).map(|idx| Instr::Br(idx)),
        TokenKind::BrIf => get_index(v, &context.labels).map(|idx| Instr::BrIf(idx)),
        TokenKind::BrTable => make_br_table::<_, LabelIndex>(v, &context.labels),
        TokenKind::Return => Some(Instr::Return),
        TokenKind::Call => get_index(v, &context.funcs).map(|idx| Instr::Call(idx)),
        TokenKind::CallIndirect => make_call_indirect(v, context),

        // Parametric Instructions
        TokenKind::Drop => Some(Instr::Drop),
        TokenKind::Select => Some(Instr::Select),

        // Variable Instructions
        TokenKind::GetLocal => get_index(v, &context.locals).map(|idx| Instr::GetLocal(idx)),
        TokenKind::SetLocal => get_index(v, &context.locals).map(|idx| Instr::SetLocal(idx)),
        TokenKind::TeeLocal => get_index(v, &context.locals).map(|idx| Instr::TeeLocal(idx)),
        TokenKind::GetGlobal => get_index(v, &context.globals).map(|idx| Instr::GetGlobal(idx)),
        TokenKind::SetGlobal => get_index(v, &context.globals).map(|idx| Instr::SetGlobal(idx)),

        // Memory Instructions
        TokenKind::I32Load => Some(Instr::I32Load(MemArg::with_alignment(4))),
        TokenKind::I64Load => Some(Instr::I64Load(MemArg::with_alignment(8))),
        TokenKind::F32Load => Some(Instr::F32Load(MemArg::with_alignment(4))),
        TokenKind::F64Load => Some(Instr::F64Load(MemArg::with_alignment(8))),
        TokenKind::I32Load8S => Some(Instr::I32Load8S(MemArg::with_alignment(1))),
        TokenKind::I32Load8U => Some(Instr::I32Load8U(MemArg::with_alignment(1))),
        TokenKind::I32Load16S => Some(Instr::I32Load16S(MemArg::with_alignment(2))),
        TokenKind::I32Load16U => Some(Instr::I32Load16U(MemArg::with_alignment(2))),
        TokenKind::I64Load8S => Some(Instr::I64Load8S(MemArg::with_alignment(1))),
        TokenKind::I64Load8U => Some(Instr::I64Load8U(MemArg::with_alignment(1))),
        TokenKind::I64Load16S => Some(Instr::I64Load16S(MemArg::with_alignment(2))),
        TokenKind::I64Load16U => Some(Instr::I64Load16U(MemArg::with_alignment(2))),
        TokenKind::I64Load32S => Some(Instr::I64Load32S(MemArg::with_alignment(4))),
        TokenKind::I64Load32U => Some(Instr::I64Load32U(MemArg::with_alignment(4))),
        TokenKind::I32Store => Some(Instr::I32Store(MemArg::with_alignment(4))),
        TokenKind::I64Store => Some(Instr::I64Store(MemArg::with_alignment(8))),
        TokenKind::F32Store => Some(Instr::F32Store(MemArg::with_alignment(4))),
        TokenKind::F64Store => Some(Instr::F64Store(MemArg::with_alignment(8))),
        TokenKind::I32Store8 => Some(Instr::I32Store8(MemArg::with_alignment(1))),
        TokenKind::I32Store16 => Some(Instr::I32Store16(MemArg::with_alignment(2))),
        TokenKind::I64Store8 => Some(Instr::I64Store8(MemArg::with_alignment(1))),
        TokenKind::I64Store16 => Some(Instr::I64Store16(MemArg::with_alignment(2))),
        TokenKind::I64Store32 => Some(Instr::I64Store32(MemArg::with_alignment(4))),
        TokenKind::MemorySize => Some(Instr::MemorySize),
        TokenKind::MemoryGrow => Some(Instr::MemoryGrow),

        // Numeric Instructions
        TokenKind::I32Const => make_const!(v, u32, Instr::I32Const),
        TokenKind::I64Const => make_const!(v, u64, Instr::I64Const),
        TokenKind::F32Const => make_const!(v, f32, Instr::F32Const),
        TokenKind::F64Const => make_const!(v, f64, Instr::F64Const),

        TokenKind::I32Clz => Some(Instr::I32Clz),
        TokenKind::I32Ctz => Some(Instr::I32Ctz),
        TokenKind::I32Popcnt => Some(Instr::I32Popcnt),
        TokenKind::I32Add => Some(Instr::I32Add),
        TokenKind::I32Sub => Some(Instr::I32Sub),
        TokenKind::I32Mul => Some(Instr::I32Mul),
        TokenKind::I32DivS => Some(Instr::I32DivS),
        TokenKind::I32DivU => Some(Instr::I32DivU),
        TokenKind::I32RemS => Some(Instr::I32RemS),
        TokenKind::I32RemU => Some(Instr::I32RemU),
        TokenKind::I32And => Some(Instr::I32And),
        TokenKind::I32Or => Some(Instr::I32Or),
        TokenKind::I32Xor => Some(Instr::I32Xor),
        TokenKind::I32Shl => Some(Instr::I32Shl),
        TokenKind::I32ShrS => Some(Instr::I32ShrS),
        TokenKind::I32ShrU => Some(Instr::I32ShrU),
        TokenKind::I32Rotl => Some(Instr::I32Rotl),
        TokenKind::I32Rotr => Some(Instr::I32Rotr),

        TokenKind::I64Clz => Some(Instr::I64Clz),
        TokenKind::I64Ctz => Some(Instr::I64Ctz),
        TokenKind::I64Popcnt => Some(Instr::I64Popcnt),
        TokenKind::I64Add => Some(Instr::I64Add),
        TokenKind::I64Sub => Some(Instr::I64Sub),
        TokenKind::I64Mul => Some(Instr::I64Mul),
        TokenKind::I64DivS => Some(Instr::I64DivS),
        TokenKind::I64DivU => Some(Instr::I64DivU),
        TokenKind::I64RemS => Some(Instr::I64RemS),
        TokenKind::I64RemU => Some(Instr::I64RemU),
        TokenKind::I64And => Some(Instr::I64And),
        TokenKind::I64Or => Some(Instr::I64Or),
        TokenKind::I64Xor => Some(Instr::I64Xor),
        TokenKind::I64Shl => Some(Instr::I64Shl),
        TokenKind::I64ShrS => Some(Instr::I64ShrS),
        TokenKind::I64ShrU => Some(Instr::I64ShrU),
        TokenKind::I64Rotl => Some(Instr::I64Rotl),
        TokenKind::I64Rotr => Some(Instr::I64Rotr),

        TokenKind::F32Abs => Some(Instr::F32Abs),
        TokenKind::F32Neg => Some(Instr::F32Neg),
        TokenKind::F32Ceil => Some(Instr::F32Ceil),
        TokenKind::F32Floor => Some(Instr::F32Floor),
        TokenKind::F32Trunc => Some(Instr::F32Trunc),
        TokenKind::F32Nearest => Some(Instr::F32Nearest),
        TokenKind::F32Sqrt => Some(Instr::F32Sqrt),
        TokenKind::F32Add => Some(Instr::F32Add),
        TokenKind::F32Sub => Some(Instr::F32Sub),
        TokenKind::F32Mul => Some(Instr::F32Mul),
        TokenKind::F32Div => Some(Instr::F32Div),
        TokenKind::F32Min => Some(Instr::F32Min),
        TokenKind::F32Max => Some(Instr::F32Max),
        TokenKind::F32Copysign => Some(Instr::F32Copysign),

        TokenKind::F64Abs => Some(Instr::F64Abs),
        TokenKind::F64Neg => Some(Instr::F64Neg),
        TokenKind::F64Ceil => Some(Instr::F64Ceil),
        TokenKind::F64Floor => Some(Instr::F64Floor),
        TokenKind::F64Trunc => Some(Instr::F64Trunc),
        TokenKind::F64Nearest => Some(Instr::F64Nearest),
        TokenKind::F64Sqrt => Some(Instr::F64Sqrt),
        TokenKind::F64Add => Some(Instr::F64Add),
        TokenKind::F64Sub => Some(Instr::F64Sub),
        TokenKind::F64Mul => Some(Instr::F64Mul),
        TokenKind::F64Div => Some(Instr::F64Div),
        TokenKind::F64Min => Some(Instr::F64Min),
        TokenKind::F64Max => Some(Instr::F64Max),
        TokenKind::F64Copysign => Some(Instr::F64Copysign),

        TokenKind::I32Eqz => Some(Instr::I32Eqz),
        TokenKind::I32Eq => Some(Instr::I32Eq),
        TokenKind::I32Ne => Some(Instr::I32Ne),
        TokenKind::I32LtS => Some(Instr::I32LtS),
        TokenKind::I32LtU => Some(Instr::I32LtU),
        TokenKind::I32GtS => Some(Instr::I32GtS),
        TokenKind::I32GtU => Some(Instr::I32GtU),
        TokenKind::I32LeS => Some(Instr::I32LeS),
        TokenKind::I32LeU => Some(Instr::I32LeU),
        TokenKind::I32GeS => Some(Instr::I32GeS),
        TokenKind::I32GeU => Some(Instr::I32GeU),

        TokenKind::I64Eqz => Some(Instr::I64Eqz),
        TokenKind::I64Eq => Some(Instr::I64Eq),
        TokenKind::I64Ne => Some(Instr::I64Ne),
        TokenKind::I64LtS => Some(Instr::I64LtS),
        TokenKind::I64LtU => Some(Instr::I64LtU),
        TokenKind::I64GtS => Some(Instr::I64GtS),
        TokenKind::I64GtU => Some(Instr::I64GtU),
        TokenKind::I64LeS => Some(Instr::I64LeS),
        TokenKind::I64LeU => Some(Instr::I64LeU),
        TokenKind::I64GeS => Some(Instr::I64GeS),
        TokenKind::I64GeU => Some(Instr::I64GeU),

        TokenKind::F32Eq => Some(Instr::F32Eq),
        TokenKind::F32Ne => Some(Instr::F32Ne),
        TokenKind::F32Lt => Some(Instr::F32Lt),
        TokenKind::F32Gt => Some(Instr::F32Gt),
        TokenKind::F32Le => Some(Instr::F32Le),
        TokenKind::F32Ge => Some(Instr::F32Ge),

        TokenKind::F64Eq => Some(Instr::F64Eq),
        TokenKind::F64Ne => Some(Instr::F64Ne),
        TokenKind::F64Lt => Some(Instr::F64Lt),
        TokenKind::F64Gt => Some(Instr::F64Gt),
        TokenKind::F64Le => Some(Instr::F64Le),
        TokenKind::F64Ge => Some(Instr::F64Ge),

        TokenKind::I32WrapToI64 => Some(Instr::I32WrapToI64),
        TokenKind::I32TruncSToF32 => Some(Instr::I32TruncSToF32),
        TokenKind::I32TruncUToF32 => Some(Instr::I32TruncUToF32),
        TokenKind::I32TruncSToF64 => Some(Instr::I32TruncSToF64),
        TokenKind::I32TruncUToF64 => Some(Instr::I32TruncUToF64),
        TokenKind::I64ExtendSToI32 => Some(Instr::I64ExtendSToI32),
        TokenKind::I64ExtendUToI32 => Some(Instr::I64ExtendUToI32),
        TokenKind::I64TruncSToF32 => Some(Instr::I64TruncSToF32),
        TokenKind::I64TruncUToF32 => Some(Instr::I64TruncUToF32),
        TokenKind::I64TruncSToF64 => Some(Instr::I64TruncSToF64),
        TokenKind::I64TruncUToF64 => Some(Instr::I64TruncUToF64),
        TokenKind::F32ConvertSToI32 => Some(Instr::F32ConvertSToI32),
        TokenKind::F32ConvertUToI32 => Some(Instr::F32ConvertUToI32),
        TokenKind::F32ConvertSToI64 => Some(Instr::F32ConvertSToI64),
        TokenKind::F32ConvertUToI64 => Some(Instr::F32ConvertUToI64),
        TokenKind::F32DemoteToF64 => Some(Instr::F32DemoteToF64),
        TokenKind::F64ConvertSToI32 => Some(Instr::F64ConvertSToI32),
        TokenKind::F64ConvertUToI32 => Some(Instr::F64ConvertUToI32),
        TokenKind::F64ConvertSToI64 => Some(Instr::F64ConvertSToI64),
        TokenKind::F64ConvertUToI64 => Some(Instr::F64ConvertUToI64),
        TokenKind::F64PromoteToF32 => Some(Instr::F64PromoteToF32),
        TokenKind::I32ReinterpretToF32 => Some(Instr::I32ReinterpretToF32),
        TokenKind::I64ReinterpretToF64 => Some(Instr::I64ReinterpretToF64),
        TokenKind::F32ReinterpretToI32 => Some(Instr::F32ReinterpretToI32),
        TokenKind::F64ReinterpretToI64 => Some(Instr::F64ReinterpretToI64),

        _ => None,
    }
}

fn get_index<'a, Iter, T>(v: &mut Iter, indices: &Vec<Option<Id>>) -> Option<T>
    where Iter: Iterator<Item=&'a CST>,
          T: FromStr + TryFrom<usize> {

    v.next()
    .and_then(|cst| cst.token())
    .and_then(|token| make_idx::<T>(token, indices).ok())
}
