use std::collections::HashMap;
use serde::{self, Deserialize, Serialize};
use crate::{BnumGroup, Manager};
pub use crate::comps::actor::perception::Perception;

#[derive(Deserialize, Serialize)]
pub struct ActorState {
    /// The bnum_grp of the actor.
    pub personality: BnumGroup,
    /// How the actor perceives the world at large.
    pub accordance: Perception,
    /// How the actor perceives themselves.
    pub self_perceptions: Perception,
    /// How the actor perceives others.
    pub perceptions: HashMap<String, Perception>,
    /// The emotional instability of the actor.
    pub emotional_variance: f32,
    /// The current emotional state of the actor.
    pub emotions: Option<(f32, String)>,
}

impl ActorState {
    /// Get effective personality, with emotions taken into account.
    pub fn get_eff_personality(&self) -> BnumGroup {
        match &self.emotions {
            Some((intensity, emotion)) => {
                Manager::get_emotion_def(emotion)
                    .apply_to_personality(self.personality, *intensity)
            },
            None => self.personality,
        }
    }
}

