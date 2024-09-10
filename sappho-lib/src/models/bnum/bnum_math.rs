use std::ops::{Add, Sub, Mul};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use crate::BNumber;

impl BNumber {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(self, other: Self, pos: f32) -> Self {
        assert!((-1f32..=1f32).contains(&pos));
        BNumber::new(self.val * (1f32 - pos) + other.val * pos)
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

impl Mul<f32> for BNumber {
    type Output = BNumber;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.val * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::BNumber;
    use crate::bnum;

    #[test]
    fn bnum_blend() {
        let bnum1 = bnum!(0.4f32);
        let bnum2 = bnum!(0.8f32);
        let blended = bnum1.blend_with(bnum2, 0.5f32);
        assert_eq!(blended.val, 0.6f32);
    }

    #[test]
    fn bnum_add() {
        let bnum1 = bnum!(0.5f32);
        let bnum2 = bnum!(0.75f32);
        let added = bnum1 + bnum2;
        assert_eq!(added.val, 0.8f32);
    }
}
