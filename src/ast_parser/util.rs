macro_rules! tk { ($kind:pat) => { Tree::Leaf(Annot{value: $kind, ..}) } }

macro_rules! kw { ($kw:pat) => { Tree::Leaf(Annot{value: TokenKind::Keyword($kw), ..}) } }

macro_rules! make_optional_id {
    ($peekable:ident) => {
        $peekable.peek()
        .and_then(|token| token.id())        
        .map(|id| {
            $peekable.next();
            id.clone()
        })
    }
}