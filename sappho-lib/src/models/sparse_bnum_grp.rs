mod sparse_bnum_grp_math;
mod sparse_bnum_grp_traits;

use crate::consts::SparseBnumGroupT;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
/// Personality model.
pub struct SparseBnumGroup {
    pub values: SparseBnumGroupT,
}

impl SparseBnumGroup {
    pub fn new(values: SparseBnumGroupT) -> Self {
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use crate::{sparse_bnum_grp, SparseBNumber, SparseBnumGroup};

    #[test]
    fn sparse_bnum_grp_serialize() {
        let personality = sparse_bnum_grp!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        let serialized = ron::to_string(&personality).unwrap();
        assert_eq!(serialized, "(values:(0.25,-0.5,-0.75,0.0))")
    }

    #[test]
    fn sparse_bnum_grp_deserialize() {
        let serialized = "(values:(0.25,-0.5,-0.75,0.0))";
        let deserialized: SparseBnumGroup = ron::from_str(serialized).unwrap();
        let personality = sparse_bnum_grp!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        assert_eq!(personality.values, deserialized.values);
    }
}
