use std::fmt::{Display, Formatter};
use crate::models::bnum::BNumber;

impl Display for BNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}