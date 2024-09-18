use crate::{BNumber, SparseBNumber};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Mul, Neg, Sub};

impl BNumber {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(self, other: Self, pos: f32) -> Self {
        assert!((-1f32..=1f32).contains(&pos));
        Self::new(self.val * (1f32 - pos) + other.val * pos)
    }

    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with_sparse(self, other: SparseBNumber, pos: f32) -> Self {
        assert!((-1f32..=1f32).contains(&pos));
        let other_value: Option<f32> = Option::from(other);
        match other_value {
            None => self,
            Some(other_value) => {
                Self::new(self.val * (1f32 - pos) + other_value * pos)
            }
        }
    }
}

impl Add for BNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::bound(self.src + rhs.src)
    }
}

impl Add<SparseBNumber> for BNumber {
    type Output = Self;

    fn add(self, rhs: SparseBNumber) -> Self::Output {
        match rhs.get_unbounded() {
            None => self,
            Some(rhs_src) => Self::bound(self.src + rhs_src),
        }
    }
}

impl Sub for BNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::bound(self.src - rhs.src)
    }
}

impl Sub<SparseBNumber> for BNumber {
    type Output = Self;

    fn sub(self, rhs: SparseBNumber) -> Self::Output {
        match rhs.get_unbounded() {
            None => self,
            Some(rhs_src) => Self::bound(self.src - rhs_src),
        }
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
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.val * rhs)
    }
}

impl Neg for BNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        BNumber::new(-self.val)
    }
}

#[cfg(test)]
mod tests {
    use crate::bnum;
    use crate::BNumber;

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
