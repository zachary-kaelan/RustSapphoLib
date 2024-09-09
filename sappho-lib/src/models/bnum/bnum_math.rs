use std::ops::{Add, Sub};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use crate::models::bnum::BNumber;

impl BNumber {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(self, other: Self, pos: f32) -> Self {
        assert!((-1f32..=1f32).contains(&pos));
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

impl PartialEq for BNumber {
    fn eq(&self, other: &Self) -> bool {
        self.src.eq(&other.src)
    }
}

impl PartialOrd for BNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.src.partial_cmp(&other.src)
    }
}
