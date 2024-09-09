use std::ops::{Add, Sub};
use crate::math::bnum::BNumber;

impl BNumber {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(self, other: Self, pos: f32) -> BNumber {
        assert!(1f32 >= pos && pos >= -1f32);
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