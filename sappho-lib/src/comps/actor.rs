mod perception;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{Personality, BNumber, personality};
use perception::Perception;


#[derive(Deserialize, Serialize)]
pub struct Actor {
    pub id: String,
    pub display_name: String,
    personality: Personality,
    accordance: Perception,
    self_perceptions: Perception,
    perceptions: HashMap<String, Perception>,
    emotional_variance: f32,
    emotions: Option<Personality>,
    cur_stage: Option<String>
}

impl Actor {
    ///
    ///
    /// # Arguments
    ///
    /// * `id`: The unique id for the actor.
    /// * `display_name`: The user-facing name for the actor.
    /// * `personality`: The personality traits of the actor.
    /// * `accordance`: The accordance values (global perceptions) of the actor.
    /// * `self_perceptions`: The self-perceptions of the actor.
    /// * `perceptions`: The initial perceptions of the actor.
    ///
    /// returns: Actor
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(id: &str, display_name: &str, personality: Option<Personality>,
               accordance: Option<Perception>, self_perceptions: Option<Perception>,
               perceptions: Option<HashMap<String, Perception>>, emotional_variance: Option<f32>) -> Self {
        Self { id: id.to_string(), display_name: display_name.to_string(),
            personality: personality.unwrap_or(personality!()),
            accordance: accordance.unwrap_or(Perception::new(None)),
            self_perceptions: self_perceptions.unwrap_or(Perception::new(personality)),
            perceptions: perceptions.unwrap_or(HashMap::new()),
            emotional_variance: emotional_variance.unwrap_or(0.5f32),
            emotions: None, cur_stage: None }
    }

    /// Get effective personality, with emotions taken into account.
    pub fn get_eff_personality(&self) -> Personality {
        match self.emotions {
            Some(e) => self.personality + e,
            None => self.personality,
        }
    }
    
    
}