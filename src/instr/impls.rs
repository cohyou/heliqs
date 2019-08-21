use std::fmt::Debug;

use super::*;

impl Debug for MemArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        if self.offset == 0 {
            write!(f, "{:?}", self.align)
        } else {
            write!(f, "{:?} offset:{:?}", self.align, self.offset)
        }
    }
}