#[derive(Debug, PartialEq)]
enum SeqHead {
    Ascii,
    Two,
    Three,
    Four,
}

// 指定のバイトがUTF-8の最初のバイトとして何に当たるのかを返す
fn head(b: u8) -> Option<SeqHead> {
    match b {
        0x00 ... 0x7F => Some(SeqHead::Ascii),
        0xC2 ... 0xDF => Some(SeqHead::Two),
        0xE0 ... 0xEF => Some(SeqHead::Three),
        0xF0 ... 0xF7 => Some(SeqHead::Four),
        _ => None,
    }
}

use std::io::{Read, Seek};
pub fn check_utf8(reader: &mut (impl Read + Seek)) -> bool {
    let mut c: &mut [u8] = &mut [0;1];
    loop {
        reader.read(&mut c)
        .map_err(|_| { return true; } )
        .map(|n| {
            if n == 0 { return false; }
            head(c[0]).map_or_else(|| { return false; }, |_| true)
        });
    }
}

#[test]
fn test() { assert_eq!(head(b')'), Some(SeqHead::Ascii)); }

#[test]
fn test2() { assert_eq!(head(0x7F), Some(SeqHead::Ascii)); }

#[test]
fn test3() { assert_eq!(head(0x80), Some(SeqHead::Two)); }
