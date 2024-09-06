pub struct BNumber {
    val: f32,
    src: f32
}

impl BNumber {
    pub fn bound(n: f32) -> BNumber {
        BNumber { val: n / (crate::math::B_MULTI + n.abs()), src: n }
    }

    pub fn unbind(&self) -> f32 { self.src }
}

pub use bnum_traits::Display;

mod bnum_traits;