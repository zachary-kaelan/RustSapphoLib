use crate::BNumber;

/// A wrapper for working with nullable bounded numbers.
#[derive(Clone, Copy, Debug)]
pub struct SparseBNumber {
    bnum: Option<BNumber>
}

impl SparseBNumber {
    /// Initializes a new `SparseBNumber` from bound number `n`.
    pub fn new(n: Option<f32>) -> Self {
        match n {
            None => Self { bnum: None },
            Some(n) => Self { bnum: Some(BNumber::new(n)) }
        }
    }

    /// Initializes a new `SparseBNumber` from unbounded number `n`.
    pub fn bound(n: Option<f32>) -> Self {
        match n {
            None => Self { bnum: None },
            Some(n) => { Self { bnum: Some(BNumber::bound(n)) }}
        }
    }

    /// Calculates the original unbounded number from `SparseBNumber` value `n`.
    pub fn unbind(n: Option<f32>) -> Option<f32> {
        match n {
            None => { None },
            Some(n) => { Some(BNumber::unbind(n)) }
        }
    }

    /// Returns the original, unbounded number.
    pub fn get_unbounded(&self) -> Option<f32> {
        match self.bnum {
            None => { None },
            Some(bnum) => { Some(bnum.get_unbounded()) }
        }
    }

    /// Returns the absolute value of the bounded number.
    pub fn get_amplitude(&self) -> Option<f32> {
        match self.bnum {
            None => { None },
            Some(bnum) => { Some(bnum.get_amplitude()) }
        }
    }
}

mod sparse_bnum_traits;
mod sparse_bnum_math;