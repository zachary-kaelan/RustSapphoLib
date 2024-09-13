mod actor_state;
mod actor_traits;
mod emotiondef;
pub mod perception;

pub use emotiondef::EmotionDef;

use crate::comps::actor::actor_state::ActorState;
use crate::comps::Stage;
use crate::{Manager, SparseBnumGroup};
use crate::{bnum_grp, BnumGroup};
pub use perception::Perception;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{RwLock, Weak};

/// A character in the storyworld.
#[derive(Deserialize, Serialize)]
pub struct Actor {
    /// The unique identifier.
    pub id: String,
    /// The user-facing name.
    pub display_name: String,
    /// The current stage the actor is on.
    pub cur_stage_id: Option<String>,

    actor_state: RwLock<ActorState>,

    #[serde(skip)]
    cur_stage: Option<Weak<Stage>>,
    #[serde(skip)]
    initialized: bool,
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
    pub fn new(
        id: String,
        display_name: String,
        personality: Option<BnumGroup>,
        accordance: Option<Perception>,
        self_perceptions: Option<Perception>,
        perceptions: Option<HashMap<String, Perception>>,
        emotional_variance: Option<f32>,
    ) -> Self {
        Self {
            id,
            display_name,
            cur_stage_id: None,
            cur_stage: None,
            initialized: true,
            actor_state: RwLock::new(ActorState {
                personality: personality.unwrap_or(bnum_grp!()),
                accordance: accordance.unwrap_or(Perception::new(None)),
                self_perceptions: self_perceptions.unwrap_or(Perception::new(None)),
                perceptions: perceptions.unwrap_or(HashMap::new()),
                emotional_variance: emotional_variance.unwrap_or(0.5f32),
                emotions: None,
            }),
        }
    }

    pub fn default(id: String, display_name: String) -> Self {
        Self::new(id, display_name, None, None, None, None, None)
    }

    pub fn init_actor(&mut self) -> bool {
        if self.initialized {
            return false;
        }

        self.cur_stage = self.cur_stage_id.as_ref().map(Manager::get_stage);

        self.initialized = true;
        true
    }
}
