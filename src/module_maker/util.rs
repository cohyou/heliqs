use std::str::FromStr;
use std::convert::TryFrom;

use core::*;
use super::error::*;

macro_rules! make_idx_func2 {
    ($token:ident,$ret:ident,$v:expr) => {{
        match &$token.value {
            TokenKind::Symbol(s) => {
                s.parse::<$ret>()
                .map_err(|_| ParseError::StrToNum($token.clone()))
            }
            TokenKind::Id(n) => {
                // println!("n: {:?}", n);
                $v.iter()
                // .inspect(|c| println!("before: {:?}", c))
                .position(|tp| if let Some(idx) = tp { idx == n } else { false })
                .and_then(|idx| $ret::try_from(idx).ok() )
                .ok_or(err!("contextから要素名が見つからない", $token))
            }
            _ => Err(err!("idxとして解釈不可", $token)),
        }
    }};
}

type Result<T> = std::result::Result<T, ParseError>;

pub fn make_idx<T>(token: &Token, indices: &Vec<Option<Id>>) -> Result<T>
    where T: FromStr + TryFrom<usize> {
    make_idx_func2!(token, T, indices)
}