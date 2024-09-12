
/// A wrapper for working with bounded numbers.
#[derive(Clone, Copy, Debug)]
pub struct BNumber {
    /// The bounded number.
    val: f32,

    /// The original unbounded number.
    src: f32
}

impl BNumber {
    /// Initializes a new `BNumber` from bound number `n`.
    pub fn new(n: f32) -> Self {
        assert!(((-1f32 + f32::EPSILON)..=(1f32 - f32::EPSILON)).contains(&n));
        Self { val: n, src: Self::unbind(n) }
    }

    /// Initializes a new `BNumber` from unbounded number `n`.
    pub fn bound(n: f32) -> Self {
        Self { val: n / (crate::models::B_MULTI + n.abs()), src: n }
    }

    /// Calculates the original unbounded number from `BNumber` value `n`.
    pub fn unbind(n: f32) -> f32 {
        assert!(((-1f32 + f32::EPSILON)..=(1f32 - f32::EPSILON)).contains(&n));
        n.signum() * (crate::models::B_MULTI / (1f32 - n.abs()) - crate::models::B_MULTI)
    }

    /// Returns the original, unbounded number.
    pub fn get_unbounded(&self) -> f32 {
        self.src
    }

    /// Returns the absolute value of the bounded number.
    pub fn get_amplitude(&self) -> f32 {
        self.val.abs()
    }
}

mod bnum_traits;
mod bnum_math;