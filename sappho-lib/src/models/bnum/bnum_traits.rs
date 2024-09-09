use std::fmt::{Display, Formatter};
use std::convert::From;
use crate::BNumber;

impl Display for BNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+.3}", self.val)
    }
}

impl From<BNumber> for f32 {
    fn from(value: BNumber) -> Self {
        value.val
    }
}