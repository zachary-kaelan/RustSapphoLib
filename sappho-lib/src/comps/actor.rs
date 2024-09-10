pub mod perception;
mod actor_traits;
mod emotiondef;

pub use emotiondef::EmotionDef;

use std::collections::HashMap;
use std::rc::Rc;
use serde::{self, Deserialize, Serialize};
use crate::{BnumGroup, BNumber, bnum_grp};
pub use perception::Perception;
use crate::managers::DefDatabase;

/// A character in the storyworld.
#[derive(Deserialize, Serialize)]
pub struct Actor {
    #[serde(skip)]
    def_database: Option<Rc<crate::managers::DefDatabase>>,

    /// The unique identifier.
    pub id: String,
    /// The user-facing name.
    pub display_name: String,
    /// The bnum_grp of the actor.
    personality: BnumGroup,
    /// How the actor perceives the world at large.
    accordance: Perception,
    /// How the actor perceives themselves.
    self_perceptions: Perception,
    /// How the actor perceives others.
    perceptions: HashMap<String, Perception>,
    /// The emotional instability of the actor.
    emotional_variance: f32,
    /// The current emotional stage of the actor.
    emotions: Option<(f32, String)>,
    /// The current stage the actor is on.
    cur_stage: Option<String>
}

impl Actor {
    ///
    ///
    /// # Arguments
    ///
    /// * `def_database`: Database of component definitions.
    /// * `id`: The unique id for the actor.
    /// * `display_name`: The user-facing name for the actor.
    /// * `bnum_grp`: The bnum_grp traits of the actor (Optional).
    /// * `accordance`: How the actor perceives the world at large (Optional).
    /// * `self_perceptions`: How the actor perceives themselves (Optional).
    /// * `perceptions`: The initial perceptions of the actor (Optional).
    /// * `emotional_variance`: The emotional instability of the actor (Optional).
    ///
    /// returns: Actor
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(def_database: Rc<DefDatabase>, id: String, display_name: String, personality: Option<BnumGroup>,
               accordance: Option<Perception>, self_perceptions: Option<Perception>,
               perceptions: Option<HashMap<String, Perception>>, emotional_variance: Option<f32>) -> Self {
        Self { def_database: Some(def_database),
            id, display_name,
            personality: personality.unwrap_or(bnum_grp!()),
            accordance: accordance.unwrap_or(Perception::new(None)),
            self_perceptions: self_perceptions.unwrap_or(Perception::new(personality)),
            perceptions: perceptions.unwrap_or(HashMap::new()),
            emotional_variance: emotional_variance.unwrap_or(0.5f32),
            emotions: None, cur_stage: None }
    }

    /// Get effective personality, with emotions taken into account.
    pub fn get_eff_personality(&self) -> BnumGroup {
        match &self.emotions {
            Some((intensity, emotion)) => {
                self.def_database.as_ref()
                    .unwrap().emotion_defs.get(emotion)
                    .unwrap().apply_to_personality(self.personality, *intensity)
            },
            None => self.personality,
        }
    }

}