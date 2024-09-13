use crate::{BNumber, SparseBNumber, SparseBnumGroup};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Mul, Neg, Sub};

impl SparseBNumber {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(self, other: Self, pos: f32) -> Self {
        if self.bnum.is_none() || other.bnum.is_none() {
            return Self { bnum: None };
        }
        Self {
            bnum: Some(self.bnum.unwrap().blend_with(other.bnum.unwrap(), pos)),
        }
    }
}

impl Add for SparseBNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.bnum.is_none() || rhs.bnum.is_none() {
            return Self { bnum: None };
        }
        Self {
            bnum: Some(self.bnum.unwrap() + rhs.bnum.unwrap()),
        }
    }
}

impl Sub for SparseBNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.bnum.is_none() || rhs.bnum.is_none() {
            return Self { bnum: None };
        }
        Self {
            bnum: Some(self.bnum.unwrap() - rhs.bnum.unwrap()),
        }
    }
}

impl PartialEq for SparseBNumber {
    fn eq(&self, other: &Self) -> bool {
        if self.bnum.is_none() != other.bnum.is_none() {
            return false;
        }
        if self.bnum.is_none() && other.bnum.is_none() {
            return true;
        }
        self.bnum.unwrap() == other.bnum.unwrap()
    }
}

impl PartialOrd for SparseBNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.bnum.is_none() != other.bnum.is_none() {
            return if self.bnum.is_none() {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            };
        }
        if self.bnum.is_none() && other.bnum.is_none() {
            return Some(Ordering::Equal);
        }
        self.bnum?.partial_cmp(&other.bnum?)
    }
}

impl Mul<f32> for SparseBNumber {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        match self.bnum {
            None => Self { bnum: None },
            Some(bnum) => Self {
                bnum: Some(bnum * rhs),
            },
        }
    }
}

impl Mul<Option<f32>> for SparseBNumber {
    type Output = Self;

    fn mul(self, rhs: Option<f32>) -> Self::Output {
        if self.bnum.is_none() || rhs.is_none() {
            return SparseBNumber::new(None);
        }
        SparseBNumber {
            bnum: Some(self.bnum.unwrap() * rhs.unwrap()),
        }
    }
}

impl Add<SparseBNumber> for BNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.bnum.is_none() || rhs.bnum.is_none() {
            return Self { bnum: None };
        }
        Self {
            bnum: Some(self.bnum.unwrap() + rhs.bnum.unwrap()),
        }
    }
}

impl Sub<SparseBNumber> for BNumber {
    type Output = Self;

    fn sub(self, rhs: SparseBNumber) -> Self::Output {
        let rhs: Option<f32> = Option::from(rhs);
        match rhs {
            None => self,
            Some(rhs) => {}
        }
        if self.bnum.is_none() || rhs.bnum.is_none() {
            return Self { bnum: None };
        }
        Self {
            bnum: Some(self.bnum.unwrap() - rhs.bnum.unwrap()),
        }
    }
}

impl Neg for SparseBNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        SparseBNumber::new(match self.bnum {
            None => None,
            Some(bnum) => Some(-f32::from(bnum)),
        })
    }
}
