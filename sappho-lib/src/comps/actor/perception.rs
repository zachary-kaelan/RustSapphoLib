
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{Personality, BNumber, personality};

/// An actor's perception of themselves, another actor, or the world at large.
#[derive(Deserialize, Serialize)]
pub struct Perception {
    /// The actor's perception of the target's personality.
    personality: Personality,
    /// The actor's perception of the target's emotional state.
    emotions: Personality,
    /// How many of each verb the actor has perceived from the target.
    observed_verbs: HashMap<String, u32>
}

impl Perception {
    pub fn new(personality: Option<Personality>) -> Self {
        Self {
            personality: personality.unwrap_or(personality!()),
            emotions: personality!(),
            observed_verbs: HashMap::new()
        }
    }
}