use crate::consts::BNUM_GROUP_SIZE;
use crate::{BNumber, BnumGroup, SparseBNumber};
use std::ops::{Add, Mul, Sub};

impl BnumGroup {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(&self, other: &Self, pos: f32) -> Self {
        let new_values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(value, other_value)| value.blend_with(*other_value, pos))
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }

    /// Sum the traits of a bnum_grp model to a single bounded value.
    pub fn sum(&self) -> BNumber {
        self.values.into_iter().reduce(|x, x1| x + x1).unwrap()
    }

    /// Sum the amplitudes of a bnum_grp model to a single bounded value.
    pub fn sum_amp(&self) -> BNumber {
        BNumber::bound(
            self.values
                .into_iter()
                .map(|x| x.get_unbounded().abs())
                .sum(),
        )
    }
}

impl Add for BnumGroup {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(value, other_value)| *value + *other_value)
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}

impl Sub for BnumGroup {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let new_values = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(value, other_value)| *value - *other_value)
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}

impl Mul<f32> for BnumGroup {
    type Output = BnumGroup;

    fn mul(self, rhs: f32) -> Self::Output {
        let new_values = self
            .values
            .iter()
            .map(|value| *value * rhs)
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}

impl Mul<[f32; BNUM_GROUP_SIZE]> for BnumGroup {
    type Output = BnumGroup;

    fn mul(self, rhs: [f32; BNUM_GROUP_SIZE]) -> Self::Output {
        let new_values = self
            .values
            .iter()
            .zip(rhs.iter())
            .map(|(value, other_value)| *value * *other_value)
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}
