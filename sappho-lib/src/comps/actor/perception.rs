
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{BnumGroup, bnum_grp};

/// An actor's perception of themselves, another actor, or the world at large.
#[derive(Deserialize, Serialize)]
pub struct Perception {
    /// The actor's perception of the target's bnum_grp.
    personality: BnumGroup,
    /// The actor's perception of the target's emotional state.
    emotions: BnumGroup,
    /// How many of each verb the actor has perceived from the target.
    observed_verbs: HashMap<String, u32>
}

impl Perception {
    pub fn new(personality: Option<BnumGroup>) -> Self {
        Self {
            personality: personality.unwrap_or(bnum_grp!()),
            emotions: bnum_grp!(),
            observed_verbs: HashMap::new()
        }
    }
}