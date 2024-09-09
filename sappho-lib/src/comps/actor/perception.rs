
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{Personality, BNumber, personality};

#[derive(Deserialize, Serialize)]
pub struct Perception {
    personality: Personality,
    emotions: Personality,
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