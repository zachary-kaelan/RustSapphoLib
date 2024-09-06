pub struct BNumber {
    val: f32,
    src: f32
}

impl BNumber {
    pub fn bound(n: f32) -> BNumber {
        BNumber { val: n / (crate::math::B_MULTI + n.abs()), src: n }
    }

    pub fn unbind(n: f32) -> f32 {
        assert!(1f32 > n && n > -1f32);
        n.signum() * (crate::math::B_MULTI / (1f32 - n.abs()) - crate::math::B_MULTI)
    }
}

mod bnum_traits;
mod bnum_math;