mod personality_math;
mod personality_traits;

use serde::{Deserialize, Serialize};
use crate::{PersonalityT};

#[derive(Clone, Copy, Deserialize, Serialize)]
/// Personality model.
pub struct Personality {
    pub values: PersonalityT
}

impl Personality {
    pub fn new(values: PersonalityT) -> Self {
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BNumber, Personality, personality};

    #[test]
    fn personality_serialize() {
        let personality = personality!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        let serialized = ron::to_string(&personality).unwrap();
        assert_eq!(serialized, "(values:(0.25,-0.5,-0.75,0.0))")
    }

    #[test]
    fn personality_deserialize() {
        let serialized = "(values:(0.25,-0.5,-0.75,0.0))";
        let deserialized: Personality = ron::from_str(serialized).unwrap();
        let personality = personality!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        assert_eq!(personality.values, deserialized.values);
    }
}