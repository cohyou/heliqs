use instr::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Module,

    Type,
    Import,
    Func,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Elem,
    Data,

    Local,
    Param,
    Result,
    AnyFunc,
    Mutable,
    Offset,

    ValType(ValType)
}

pub(super) fn vec_to_keyword(s: &[u8]) -> Option<Keyword> {
    match s {
        b"module" => Some(Keyword::Module),

        b"type" => Some(Keyword::Type),
        b"import" => Some(Keyword::Import),
        b"func" => Some(Keyword::Func),
        b"table" => Some(Keyword::Table),
        b"memory" => Some(Keyword::Memory),
        b"global" => Some(Keyword::Global),
        b"export" => Some(Keyword::Export),
        b"start" => Some(Keyword::Start),
        b"elem" => Some(Keyword::Elem),
        b"data" => Some(Keyword::Data),

        b"local" => Some(Keyword::Local),
        b"param" => Some(Keyword::Param),
        b"result" => Some(Keyword::Result),
        b"anyfunc" => Some(Keyword::AnyFunc),
        b"mut" => Some(Keyword::Mutable),
        b"offset" => Some(Keyword::Offset),

        b"i32" | b"i64" | b"f32" | b"f64" => Some(Keyword::ValType(vec_to_valtype(s))),

        _ => None,
    }
}

fn vec_to_valtype(s: &[u8]) -> ValType {
    match s {
        b"i32" => ValType::I32,
        b"i64" => ValType::I64,
        b"f32" => ValType::F32,
        b"f64" => ValType::F64,
        _ => panic!("vec_to_valtype"),
    }
}

macro_rules! make_token {
    ($bytes:ident) => {
        if !$bytes.is_empty() {
            let s = String::from_utf8($bytes.to_vec()).unwrap();
            let t = match s.as_ref() {
                "module" => Token::module(Loc(0, 0)),

                "type" => Token::func_type(Loc(0, 0)),
                "import" => Token::import(Loc(0, 0)),
                "func" => Token::func(Loc(0, 0)),
                "table" => Token::table(Loc(0, 0)),
                "memory" => Token::memory(Loc(0, 0)),
                "global" => Token::global(Loc(0, 0)),
                "export" => Token::export(Loc(0, 0)),
                "start" => Token::start(Loc(0, 0)),
                "elem" => Token::elem(Loc(0, 0)),
                "data" => Token::data(Loc(0, 0)),

                "local" => Token::local(Loc(0, 0)),
                "param" => Token::param(Loc(0, 0)),
                "result" => Token::func_result(Loc(0, 0)),
                "anyfunc" => Token::any_func(Loc(0, 0)),
                "mut" => Token::mutable(Loc(0, 0)),
                "offset" => Token::offset(Loc(0, 0)),

                "inf" => Token::infinity(Loc(0, 0)),
                "nan" => Token::nan(Loc(0, 0)),  // "nan:0x"も必要


                "i32" => Token::val_type(ValType::I32, Loc(0, 0)),
                "i64" => Token::val_type(ValType::I64, Loc(0, 0)),
                "f32" => Token::val_type(ValType::F32, Loc(0, 0)),
                "f64" => Token::val_type(ValType::F64, Loc(0, 0)),

                "block" => Token::block(Loc(0, 0)),
                "loop" => Token::r#loop(Loc(0, 0)),
                "if" => Token::r#if(Loc(0, 0)),
                "else" => Token::r#else(Loc(0, 0)),
                "end" => Token::end(Loc(0, 0)),

                "unreachable" => Token::unreachable(Loc(0, 0)),
                "nop" => Token::nop(Loc(0, 0)),
                "br" => Token::br(Loc(0, 0)),
                "br_if" => Token::br_if(Loc(0, 0)),
                "br_table" => Token::br_table(Loc(0, 0)),
                "return" => Token::r#return(Loc(0, 0)),
                "call" => Token::call(Loc(0, 0)),
                "call_indirect" => Token::call_indirect(Loc(0, 0)),

                "drop" => Token::drop(Loc(0, 0)),
                "select" => Token::select(Loc(0, 0)),

                "get_local" => Token::get_local(Loc(0, 0)),
                "set_local" => Token::set_local(Loc(0, 0)),
                "tee_local" => Token::tee_local(Loc(0, 0)),
                "get_global" => Token::get_global(Loc(0, 0)),
                "set_global" => Token::set_global(Loc(0, 0)),

                // Memory Instructions
                "i32.load" => Token::i32_load(Loc(0, 0)),
                "i64.load" => Token::i64_load(Loc(0, 0)),
                "f32.load" => Token::f32_load(Loc(0, 0)),
                "f64.load" => Token::f64_load(Loc(0, 0)),
                "i32.load8_s" => Token::i32_load8_s(Loc(0, 0)),
                "i32.load8_u" => Token::i32_load8_u(Loc(0, 0)),
                "i32.load16_s" => Token::i32_load16_s(Loc(0, 0)),
                "i32.load16_u" => Token::i32_load16_u(Loc(0, 0)),
                "i64.load8_s" => Token::i64_load8_s(Loc(0, 0)),
                "i64.load8_u" => Token::i64_load8_u(Loc(0, 0)),
                "i64.load16_s" => Token::i64_load16_s(Loc(0, 0)),
                "i64.load16_u" => Token::i64_load16_u(Loc(0, 0)),
                "i64.load32_s" => Token::i64_load32_s(Loc(0, 0)),
                "i64.load32_u" => Token::i64_load32_u(Loc(0, 0)),
                "i32.store" => Token::i32_store(Loc(0, 0)),
                "i64.store" => Token::i64_store(Loc(0, 0)),
                "f32.store" => Token::f32_store(Loc(0, 0)),
                "f64.store" => Token::f64_store(Loc(0, 0)),
                "i32.store8" => Token::i32_store8(Loc(0, 0)),
                "i32.store16" => Token::i32_store16(Loc(0, 0)),
                "i64.store8" => Token::i64_store8(Loc(0, 0)),
                "i64.store16" => Token::i64_store16(Loc(0, 0)),
                "i64.store32" => Token::i64_store32(Loc(0, 0)),
                "memory.size" => Token::memory_size(Loc(0, 0)),
                "memory.grow" => Token::memory_grow(Loc(0, 0)),

                // Numeric Instructions
                "i32.const" => Token::i32_const(Loc(0, 0)),
                "i64.const" => Token::i64_const(Loc(0, 0)),
                "f32.const" => Token::f32_const(Loc(0, 0)),
                "f64.const" => Token::f64_const(Loc(0, 0)),

                "i32.clz" => Token::i32_clz(Loc(0, 0)),
                "i32.ctz" => Token::i32_ctz(Loc(0, 0)),
                "i32.popcnt" => Token::i32_popcnt(Loc(0, 0)),
                "i32.add" => Token::i32_add(Loc(0, 0)),
                "i32.sub" => Token::i32_sub(Loc(0, 0)),
                "i32.mul" => Token::i32_mul(Loc(0, 0)),
                "i32.div_s" => Token::i32_div_s(Loc(0, 0)),
                "i32.div_u" => Token::i32_div_u(Loc(0, 0)),
                "i32.rem_s" => Token::i32_rem_s(Loc(0, 0)),
                "i32.rem_u" => Token::i32_rem_u(Loc(0, 0)),
                "i32.and" => Token::i32_and(Loc(0, 0)),
                "i32.or" => Token::i32_or(Loc(0, 0)),
                "i32.xor" => Token::i32_xor(Loc(0, 0)),
                "i32.shl" => Token::i32_shl(Loc(0, 0)),
                "i32.shr_s" => Token::i32_shr_s(Loc(0, 0)),
                "i32.shr_u" => Token::i32_shr_u(Loc(0, 0)),
                "i32.rotl" => Token::i32_rotl(Loc(0, 0)),
                "i32.rotr" => Token::i32_rotr(Loc(0, 0)),

                "i64.clz" => Token::i64_clz(Loc(0, 0)),
                "i64.ctz" => Token::i64_ctz(Loc(0, 0)),
                "i64.popcnt" => Token::i64_popcnt(Loc(0, 0)),
                "i64.add" => Token::i64_add(Loc(0, 0)),
                "i64.sub" => Token::i64_sub(Loc(0, 0)),
                "i64.mul" => Token::i64_mul(Loc(0, 0)),
                "i64.div_s" => Token::i64_div_s(Loc(0, 0)),
                "i64.div_u" => Token::i64_div_u(Loc(0, 0)),
                "i64.rem_s" => Token::i64_rem_s(Loc(0, 0)),
                "i64.rem_u" => Token::i64_rem_u(Loc(0, 0)),
                "i64.and" => Token::i64_and(Loc(0, 0)),
                "i64.or" => Token::i64_or(Loc(0, 0)),
                "i64.xor" => Token::i64_xor(Loc(0, 0)),
                "i64.shl" => Token::i64_shl(Loc(0, 0)),
                "i64.shr_s" => Token::i64_shr_s(Loc(0, 0)),
                "i64.shr_u" => Token::i64_shr_u(Loc(0, 0)),
                "i64.rotl" => Token::i64_rotl(Loc(0, 0)),
                "i64.rotr" => Token::i64_rotr(Loc(0, 0)),

                "f32.abs" => Token::f32_abs(Loc(0, 0)),
                "f32.neg" => Token::f32_neg(Loc(0, 0)),
                "f32.ceil" => Token::f32_ceil(Loc(0, 0)),
                "f32.floor" => Token::f32_floor(Loc(0, 0)),
                "f32.trunc" => Token::f32_trunc(Loc(0, 0)),
                "f32.nearest" => Token::f32_nearest(Loc(0, 0)),
                "f32.sqrt" => Token::f32_sqrt(Loc(0, 0)),
                "f32.add" => Token::f32_add(Loc(0, 0)),
                "f32.sub" => Token::f32_sub(Loc(0, 0)),
                "f32.mul" => Token::f32_mul(Loc(0, 0)),
                "f32.div" => Token::f32_div(Loc(0, 0)),
                "f32.min" => Token::f32_min(Loc(0, 0)),
                "f32.max" => Token::f32_max(Loc(0, 0)),
                "f32.copysign" => Token::f32_copysign(Loc(0, 0)),

                "f64.abs" => Token::f64_abs(Loc(0, 0)),
                "f64.neg" => Token::f64_neg(Loc(0, 0)),
                "f64.ceil" => Token::f64_ceil(Loc(0, 0)),
                "f64.floor" => Token::f64_floor(Loc(0, 0)),
                "f64.trunc" => Token::f64_trunc(Loc(0, 0)),
                "f64.nearest" => Token::f64_nearest(Loc(0, 0)),
                "f64.sqrt" => Token::f64_sqrt(Loc(0, 0)),
                "f64.add" => Token::f64_add(Loc(0, 0)),
                "f64.sub" => Token::f64_sub(Loc(0, 0)),
                "f64.mul" => Token::f64_mul(Loc(0, 0)),
                "f64.div" => Token::f64_div(Loc(0, 0)),
                "f64.min" => Token::f64_min(Loc(0, 0)),
                "f64.max" => Token::f64_max(Loc(0, 0)),
                "f64.copysign" => Token::f64_copysign(Loc(0, 0)),

                "i32.eqz" => Token::i32_eqz(Loc(0, 0)),
                "i32.eq" => Token::i32_eq(Loc(0, 0)),
                "i32.ne" => Token::i32_ne(Loc(0, 0)),
                "i32.lt_s" => Token::i32_lt_s(Loc(0, 0)),
                "i32.lt_u" => Token::i32_lt_u(Loc(0, 0)),
                "i32.gt_s" => Token::i32_gt_s(Loc(0, 0)),
                "i32.gt_u" => Token::i32_gt_u(Loc(0, 0)),
                "i32.le_s" => Token::i32_le_s(Loc(0, 0)),
                "i32.le_u" => Token::i32_le_u(Loc(0, 0)),
                "i32.ge_s" => Token::i32_ge_s(Loc(0, 0)),
                "i32.ge_u" => Token::i32_ge_u(Loc(0, 0)),

                "i64.eqz" => Token::i64_eqz(Loc(0, 0)),
                "i64.eq" => Token::i64_eq(Loc(0, 0)),
                "i64.ne" => Token::i64_ne(Loc(0, 0)),
                "i64.lt_s" => Token::i64_lt_s(Loc(0, 0)),
                "i64.lt_u" => Token::i64_lt_u(Loc(0, 0)),
                "i64.gt_s" => Token::i64_gt_s(Loc(0, 0)),
                "i64.gt_u" => Token::i64_gt_u(Loc(0, 0)),
                "i64.le_s" => Token::i64_le_s(Loc(0, 0)),
                "i64.le_u" => Token::i64_le_u(Loc(0, 0)),
                "i64.ge_s" => Token::i64_ge_s(Loc(0, 0)),
                "i64.ge_u" => Token::i64_ge_u(Loc(0, 0)),

                "f32.eq" => Token::f32_eq(Loc(0, 0)),
                "f32.ne" => Token::f32_ne(Loc(0, 0)),
                "f32.lt" => Token::f32_lt(Loc(0, 0)),
                "f32.gt" => Token::f32_gt(Loc(0, 0)),
                "f32.le" => Token::f32_le(Loc(0, 0)),
                "f32.ge" => Token::f32_ge(Loc(0, 0)),

                "f64.eq" => Token::f64_eq(Loc(0, 0)),
                "f64.ne" => Token::f64_ne(Loc(0, 0)),
                "f64.lt" => Token::f64_lt(Loc(0, 0)),
                "f64.gt" => Token::f64_gt(Loc(0, 0)),
                "f64.le" => Token::f64_le(Loc(0, 0)),
                "f64.ge" => Token::f64_ge(Loc(0, 0)),

                "i32.wrap/i64" => Token::i32_wrap_to_i64(Loc(0, 0)),
                "i32.trunc_s/f32" => Token::i32_trunc_s_to_f32(Loc(0, 0)),
                "i32.trunc_u/f32" => Token::i32_trunc_u_to_f32(Loc(0, 0)),
                "i32.trunc_s/f64" => Token::i32_trunc_s_to_f64(Loc(0, 0)),
                "i32.trunc_u/f64" => Token::i32_trunc_u_to_f64(Loc(0, 0)),
                "i64.extend_s/i32" => Token::i64_extend_s_to_i32(Loc(0, 0)),
                "i64.extend_u/i32" => Token::i64_extend_u_to_i32(Loc(0, 0)),
                "i64.trunc_s/f32" => Token::i64_trunc_s_to_f32(Loc(0, 0)),
                "i64.trunc_u/f32" => Token::i64_trunc_u_to_f32(Loc(0, 0)),
                "i64.trunc_s/f64" => Token::i64_trunc_s_to_f64(Loc(0, 0)),
                "i64.trunc_u/f64" => Token::i64_trunc_u_to_f64(Loc(0, 0)),
                "f32.convert_s/i32" => Token::f32_convert_s_to_i32(Loc(0, 0)),
                "f32.convert_u/i32" => Token::f32_convert_u_to_i32(Loc(0, 0)),
                "f32.convert_s/i64" => Token::f32_convert_s_to_i64(Loc(0, 0)),
                "f32.convert_u/i64" => Token::f32_convert_u_to_i64(Loc(0, 0)),
                "f32.demote/f64" => Token::f32_demote_to_f64(Loc(0, 0)),
                "f64.convert_s/i32" => Token::f64_convert_s_to_i32(Loc(0, 0)),
                "f64.convert_u/i32" => Token::f64_convert_u_to_i32(Loc(0, 0)),
                "f64.convert_s/i64" => Token::f64_convert_s_to_i64(Loc(0, 0)),
                "f64.convert_u/i64" => Token::f64_convert_u_to_i64(Loc(0, 0)),
                "f64.promote/f32" => Token::f64_promote_to_f32(Loc(0, 0)),
                "i32.reinterpret/f32" => Token::i32_reinterpret_to_f32(Loc(0, 0)),
                "i64.reinterpret/f64" => Token::i64_reinterpret_to_f64(Loc(0, 0)),
                "f32.reinterpret/i32" => Token::f32_reinterpret_to_i32(Loc(0, 0)),
                "f64.reinterpret/i64" => Token::f64_reinterpret_to_i64(Loc(0, 0)),

                _ if $bytes[0] == b'$' => Token::id(s[1..].to_string(), Loc(0, 0)),
                _ => {
                    Token::symbol(s, Loc(0, 0))
                },
            };
            Some(t)
        } else {
            None
        }
    };
}
