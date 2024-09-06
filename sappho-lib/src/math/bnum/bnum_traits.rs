use std::fmt::{Display, Formatter};
use crate::math::bnum::BNumber;

impl Display for BNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Clone for BNumber {
    fn clone(&self) -> Self {
        BNumber { val: self.val, src: self.src }
    }
}

impl Copy for BNumber {}