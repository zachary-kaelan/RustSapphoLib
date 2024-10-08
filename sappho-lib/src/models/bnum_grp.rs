mod bnum_grp_math;
mod bnum_grp_traits;

use crate::consts::BnumGroupT;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
/// Personality model.
pub struct BnumGroup {
    pub values: BnumGroupT,
}

impl BnumGroup {
    pub fn new(values: BnumGroupT) -> Self {
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bnum_grp, BNumber, BnumGroup};

    #[test]
    fn bnum_grp_serialize() {
        let personality = bnum_grp!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        let serialized = ron::to_string(&personality).unwrap();
        assert_eq!(serialized, "(values:(0.25,-0.5,-0.75,0.0))")
    }

    #[test]
    fn bnum_grp_deserialize() {
        let serialized = "(values:(0.25,-0.5,-0.75,0.0))";
        let deserialized: BnumGroup = ron::from_str(serialized).unwrap();
        let personality = bnum_grp!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        assert_eq!(personality.values, deserialized.values);
    }
}
