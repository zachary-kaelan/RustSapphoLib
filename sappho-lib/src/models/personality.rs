mod personality_math;
mod personality_traits;

use serde::{Deserialize, Serialize};
use crate::BNumber;

#[derive(Clone, Copy, Deserialize, Serialize)]
/// Personality model.
pub struct Personality {
    pub bad_good: BNumber,
    pub false_honest: BNumber,
    pub timid_dominant: BNumber,
    pub ascetic_hedonistic: BNumber
}

// Define a macro to generate the new function
macro_rules! impl_personality_new {
    ($($field:ident),*) => {
        impl Personality {
            pub fn new($($field: BNumber),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}

impl_personality_new!(bad_good, false_honest, timid_dominant, ascetic_hedonistic);

#[cfg(test)]
mod tests {
    use crate::{BNumber, Personality, personality};

    #[test]
    fn personality_serialize() {
        let personality = personality!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        let serialized = ron::to_string(&personality).unwrap();
        assert_eq!(serialized, "(bad_good:0.25,false_honest:-0.5,timid_dominant:-0.75,ascetic_hedonistic:0.0)")
    }

    #[test]
    fn personality_deserialize() {
        let serialized = "(bad_good:0.25,false_honest:-0.5,timid_dominant:-0.75,ascetic_hedonistic:0.0)";
        let deserialized: Personality = ron::from_str(serialized).unwrap();
        let personality = personality!(0.25f32, -0.5f32, -0.75f32, 0.0f32);
        assert_eq!(personality.bad_good, deserialized.bad_good);
        assert_eq!(personality.false_honest, deserialized.false_honest);
        assert_eq!(personality.timid_dominant, deserialized.timid_dominant);
        assert_eq!(personality.ascetic_hedonistic, deserialized.ascetic_hedonistic);
    }
}