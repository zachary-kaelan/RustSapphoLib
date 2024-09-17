use crate::consts::BNUM_GROUP_SIZE;
use crate::{BnumGroup, SparseBnumGroup};

pub struct EmotionDef {
    pub id: String,
    pub display_name: String,
    personality_modifiers: SparseBnumGroup,
    personality_weights: Option<[Option<f32>; BNUM_GROUP_SIZE]>,
    perception_modifiers: SparseBnumGroup,
    perception_weights: Option<[Option<f32>; BNUM_GROUP_SIZE]>,
}

impl EmotionDef {
    pub fn new(
        id: String,
        display_name: String,
        personality_modifiers: SparseBnumGroup,
        personality_weights: Option<[Option<f32>; BNUM_GROUP_SIZE]>,
        perception_modifiers: SparseBnumGroup,
        perception_weights: Option<[Option<f32>; BNUM_GROUP_SIZE]>,
    ) -> Self {
        Self {
            id,
            display_name,
            personality_modifiers,
            personality_weights,
            perception_modifiers,
            perception_weights,
        }
    }

    pub fn apply_to_personality(&self, values: BnumGroup, intensity: f32) -> BnumGroup {
        let modified_values = match self.personality_weights {
            None => values,
            Some(w) => {
                let weights: [f32; BNUM_GROUP_SIZE] = w
                    .iter()
                    .map(|x| x.unwrap_or(1.0f32) * intensity + (1.0f32 - intensity))
                    .collect::<Vec<f32>>()
                    .try_into()
                    .expect("Incorrect Length");
                values * weights
            }
        };
        modified_values + self.personality_modifiers * intensity
    }

    pub fn apply_to_perception(&self, values: BnumGroup, intensity: f32) -> BnumGroup {
        let modified_values = match self.perception_weights {
            None => values,
            Some(w) => {
                let weights: [f32; BNUM_GROUP_SIZE] = w
                    .iter()
                    .map(|x| x.unwrap_or(1.0f32) * intensity + (1.0f32 - intensity))
                    .collect::<Vec<f32>>()
                    .try_into()
                    .expect("Incorrect Length");
                values * weights
            }
        };
        modified_values + self.perception_modifiers * intensity
    }
}
