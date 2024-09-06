use std::ops::{Add, Sub};
use crate::math::bnum::BNumber;

impl BNumber {
    pub fn blend_with(self, other: Self, pos: f32) -> BNumber {
        BNumber::bound(self.val * (1f32 - pos) + other.val * pos)
    }
}

impl Add for BNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BNumber::bound(self.src + rhs.src)
    }
}

impl Sub for BNumber {
    type Output = BNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        BNumber::bound(self.src - rhs.src)
    }
}