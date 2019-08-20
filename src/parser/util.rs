
macro_rules! parse_optional_id {
    ($this:ident, $v:expr) => {
        if let tk!(TokenKind::Id(s)) = &$this.lookahead {
            let new_s = s.clone();
            $v.push(Some(new_s));
            $this.consume()?;
        } else {
            $v.push(None);
        }
    }
}