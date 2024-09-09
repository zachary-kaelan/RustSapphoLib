
/// A wrapper for working with bounded numbers.
#[derive(Clone, Copy)]
pub struct BNumber {
    /// The bounded number.
    val: f32,

    /// The original unbounded number.
    src: f32
}

impl BNumber {
    /// Initializes a new `BNumber` from bound number `n`.
    pub fn new(n: f32) -> BNumber {
        assert!(1f32 > n && n > -1f32);
        BNumber { val: n, src: BNumber::unbind(n) }
    }
    
    /// Initializes a new `BNumber` from unbounded number `n`.
    pub fn bound(n: f32) -> BNumber {
        BNumber { val: n / (crate::models::B_MULTI + n.abs()), src: n }
    }

    /// Calculates the original unbounded number from `BNumber` value `n`.
    pub fn unbind(n: f32) -> f32 {
        assert!(1f32 > n && n > -1f32);
        n.signum() * (crate::models::B_MULTI / (1f32 - n.abs()) - crate::models::B_MULTI)
    }
}

mod bnum_traits;
mod bnum_math;