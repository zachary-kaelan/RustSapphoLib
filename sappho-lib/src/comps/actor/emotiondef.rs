use crate::{BNumber, BnumGroup};
use crate::consts::BNUM_GROUP_SIZE;

pub struct EmotionDef {
    pub id: String,
    pub display_name: String,
    personality_modifiers: Option<BnumGroup>,
    personality_weights: Option<[Option<f32>; BNUM_GROUP_SIZE]>,
    perception_modifiers: Option<BnumGroup>,
    perception_weights: Option<[Option<f32>; BNUM_GROUP_SIZE]>
}

impl EmotionDef {
    pub fn apply_to_personality(&self, values: BnumGroup, intensity: f32) -> BnumGroup {
        let modified_values = match self.personality_weights {
            None => { values }
            Some(w) => { 
                let weights: [f32; BNUM_GROUP_SIZE] = w.iter()
                    .map(|x| x.unwrap_or(1.0f32) * intensity + (1.0f32 - intensity))
                    .collect::<Vec<f32>>()
                    .try_into()
                    .expect("Incorrect Length");
                values * weights
            }
        };
        match self.personality_modifiers {
            Some(m) => { (m * intensity) + modified_values }
            None => { modified_values }
        }
    }
    
    pub fn apply_to_perception(&self, values: BnumGroup, intensity: f32) -> BnumGroup {
        let modified_values = match self.perception_weights {
            None => { values }
            Some(w) => {
                let weights: [f32; BNUM_GROUP_SIZE] = w.iter()
                    .map(|x| x.unwrap_or(1.0f32) * intensity + (1.0f32 - intensity))
                    .collect::<Vec<f32>>()
                    .try_into()
                    .expect("Incorrect Length");
                values * weights
            }
        };
        match self.perception_modifiers {
            Some(m) => { (m * intensity) + modified_values }
            None => { modified_values }
        }
    }
}