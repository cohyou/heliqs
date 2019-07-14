use std::fmt::Debug;

// valtype ::= i32 | i64 | f32 | f64
#[derive(Debug, PartialEq, Clone)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

impl Default for ValType {
    fn default() -> Self {
        ValType::I32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Module,
    Import,
    Type,
    Local,
    Func,
    Param,
    FuncResult,
    ValType(ValType),
    Symbol(String),
    Name(String), // $で始まる
    Text(String), // 普通の文字列 "で囲まれている
    End,
    Empty,
}