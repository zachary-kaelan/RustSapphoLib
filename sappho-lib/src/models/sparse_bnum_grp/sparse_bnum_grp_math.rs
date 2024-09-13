use crate::consts::{SparseBnumGroupT, BNUM_GROUP_SIZE};
use crate::{SparseBNumber, SparseBnumGroup};
use std::ops::{Add, Mul, Sub};

impl SparseBnumGroup {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    pub fn blend_with(&self, other: &Self, pos: f32) -> Self {
        if self.values.is_none() || other.values.is_none() {
            return Self { values: None };
        }

        let new_values = self
            .values
            .unwrap()
            .iter()
            .zip(other.values.unwrap().iter())
            .map(|(value, other_value)| value.blend_with(*other_value, pos))
            .collect::<Vec<SparseBNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self {
            values: Some(new_values),
        }
    }

    /// Sum the traits of a bnum_grp model to a single bounded value.
    pub fn sum(&self) -> SparseBNumber {
        match self.values {
            None => SparseBNumber::new(None),
            Some(values) => values.into_iter().reduce(|x, x1| x + x1).unwrap(),
        }
    }

    /// Sum the amplitudes of a bnum_grp model to a single bounded value.
    pub fn sum_amp(&self) -> SparseBNumber {
        SparseBNumber::bound(match self.values {
            None => None,
            Some(values) => Some(
                values
                    .into_iter()
                    .filter_map(|x| match x.get_unbounded() {
                        None => None,
                        Some(x) => Some(x.abs()),
                    })
                    .sum(),
            ),
        })
    }
}

impl Add for SparseBnumGroup {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.values.is_none() || other.values.is_none() {
            return Self { values: None };
        }

        let new_values = self
            .values
            .unwrap()
            .iter()
            .zip(other.values.unwrap().iter())
            .map(|(value, other_value)| *value + *other_value)
            .collect::<Vec<SparseBNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self {
            values: Some(new_values),
        }
    }
}

impl Sub for SparseBnumGroup {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.values.is_none() || other.values.is_none() {
            return Self { values: None };
        }

        let new_values = self
            .values
            .unwrap()
            .iter()
            .zip(other.values.unwrap().iter())
            .map(|(value, other_value)| *value - *other_value)
            .collect::<Vec<SparseBNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self {
            values: Some(new_values),
        }
    }
}

impl Mul<f32> for SparseBnumGroup {
    type Output = SparseBnumGroup;

    fn mul(self, rhs: f32) -> Self::Output {
        if self.values.is_none() {
            return Self { values: None };
        }

        let new_values = self
            .values
            .unwrap()
            .iter()
            .map(|value| *value * rhs)
            .collect::<Vec<SparseBNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self {
            values: Some(new_values),
        }
    }
}

impl Mul<Option<f32>> for SparseBnumGroup {
    type Output = SparseBnumGroup;

    fn mul(self, rhs: Option<f32>) -> Self::Output {
        if rhs.is_none() {
            return Self { values: None };
        }
        let rhs = rhs.unwrap();
        self * rhs
    }
}

impl Mul<[f32; BNUM_GROUP_SIZE]> for SparseBnumGroup {
    type Output = SparseBnumGroup;

    fn mul(self, rhs: [f32; BNUM_GROUP_SIZE]) -> Self::Output {
        match self.values {
            None => Self::new(None),
            Some(values) => {
                let new_values = values
                    .iter()
                    .zip(rhs.iter())
                    .map(|(value, other_value)| *value * *other_value)
                    .collect::<Vec<SparseBNumber>>()
                    .try_into()
                    .expect("Incorrect Length");
                Self {
                    values: Some(new_values),
                }
            }
        }
    }
}

impl Mul<[Option<f32>; BNUM_GROUP_SIZE]> for SparseBnumGroup {
    type Output = SparseBnumGroup;

    fn mul(self, rhs: [Option<f32>; BNUM_GROUP_SIZE]) -> Self::Output {
        match self.values {
            None => Self::new(None),
            Some(values) => {
                let new_values = values
                    .iter()
                    .zip(rhs.iter())
                    .map(|(value, other_value)| *value * *other_value)
                    .collect::<Vec<SparseBNumber>>()
                    .try_into()
                    .expect("Incorrect Length");
                Self {
                    values: Some(new_values),
                }
            }
        }
    }
}
