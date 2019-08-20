#[macro_export]
macro_rules! p {
    ($e:expr) => { println!(concat!(stringify!($e), ": {:?}"), {&$e}); };
}

#[macro_export]
macro_rules! pp {
    ($i:expr, $e:expr) => { println!(concat!(stringify!($i), ": {:?}"), {&$e}); };
}

macro_rules! la {    
    ($this:ident) => {
        p!($this.lookahead);
    };
}

macro_rules! lla {    
    ($i:expr, $this:ident) => {
        pp!($i, $this.lookahead);
    };
}

#[macro_export]
macro_rules! tk { ($kind:pat) => { Annot{value: $kind, ..} } }

macro_rules! kw { ($kw:pat) => {
    Annot{value: TokenKind::Keyword($kw), ..}
} }

macro_rules! nm { ($nm:pat) => {
    Annot{value: TokenKind::Number($nm), ..}
} }