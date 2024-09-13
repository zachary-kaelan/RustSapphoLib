use crate::{sparse_bnum_grp, SparseBnumGroup};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An actor's perception of themselves, another actor, or the world at large.
#[derive(Deserialize, Serialize)]
pub struct Perception {
    /// The actor's perception of the target's bnum_grp.
    personality: SparseBnumGroup,
    /// The actor's perception of the target's emotional state.
    emotions: SparseBnumGroup,
    /// How many of each verb the actor has perceived from the target.
    observed_verbs: HashMap<String, u32>,
}

impl Perception {
    pub fn new(personality: Option<SparseBnumGroup>) -> Self {
        Self {
            personality: personality.unwrap_or(sparse_bnum_grp!()),
            emotions: sparse_bnum_grp!(),
            observed_verbs: HashMap::new(),
        }
    }
}
