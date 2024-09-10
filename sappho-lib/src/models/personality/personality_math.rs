use std::ops::{Add, Sub};
use crate::{BNumber, Personality};


impl Personality {
    /// Blends the bounded values of `self` and `other` based on weight `pos`.
    /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
    fn blend_with(self, other: Self, pos: f32) -> Self {
        let new_values = self.values.iter()
            .zip(other.values.iter())
            .map(|(value, other_value)| { value.blend_with(*other_value, pos) })
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}

impl Add for Personality {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_values = self.values.iter()
            .zip(other.values.iter())
            .map(|(value, other_value)| { *value + *other_value })
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}

impl Sub for Personality {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let new_values = self.values.iter()
            .zip(other.values.iter())
            .map(|(value, other_value)| { *value - *other_value })
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Incorrect Length");
        Self { values: new_values }
    }
}