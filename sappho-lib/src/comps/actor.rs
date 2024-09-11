pub mod perception;
mod actor_traits;
mod emotiondef;
mod actor_state;

pub use emotiondef::EmotionDef;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{RwLock, Weak};
use serde::{self, Deserialize, Serialize};
use crate::{BnumGroup, BNumber, bnum_grp};
pub use perception::Perception;
use crate::comps::actor::actor_state::ActorState;
use crate::comps::Stage;
use crate::Manager;

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
    initialized: bool
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
    pub fn new(id: String, display_name: String, personality: Option<BnumGroup>,
               accordance: Option<Perception>, self_perceptions: Option<Perception>,
               perceptions: Option<HashMap<String, Perception>>, emotional_variance: Option<f32>) -> Self {

        Self {
            id, display_name, cur_stage_id: None, cur_stage: None, initialized: true,
            actor_state: RwLock::new(ActorState {
                personality: personality.unwrap_or(bnum_grp!()),
                accordance: accordance.unwrap_or(Perception::new(None)),
                self_perceptions: self_perceptions.unwrap_or(Perception::new(personality)),
                perceptions: perceptions.unwrap_or(HashMap::new()),
                emotional_variance: emotional_variance.unwrap_or(0.5f32),
                emotions: None
            })}
    }

    pub fn init_actor(&mut self) -> bool {
        if self.initialized {
            return false;
        }

        self.cur_stage = match &self.cur_stage_id {
            None => None,
            Some(s) => Some(Manager::get_stage(s))
        };

        self.initialized = true;
        true
    }
}