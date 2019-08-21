
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

macro_rules! parse_optional_label_id {
    ($this:ident, $v:expr) => {
        if let tk!(TokenKind::Id(s)) = &$this.lookahead {
            let new_s = s.clone();
            $v.insert(0, Some(new_s));
            $this.consume()?;
        } else {
            $v.push(None);
        }
    }
}

macro_rules! parse_field {
    ($this:ident, $field_type:ident, $f:expr) => {
        if !$this.is_rparen()? {            
            if let tk!(TokenKind::LeftParen) = $this.lookahead {
                $this.consume()?;
            }
            loop {
                if let kw!(Keyword::$field_type) = &$this.lookahead {
                    { $f }
                    if let tk!(TokenKind::LeftParen) = $this.lookahead {
                        let peeked = $this.peek()?;
                        if let kw!(Keyword::$field_type) = peeked {
                            $this.consume()?;

                            continue;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    };
}