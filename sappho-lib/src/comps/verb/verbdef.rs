use crate::consts::BNUM_GROUP_SIZE;
use crate::BNumber;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub enum VerbType {
    Idle,
    ExpressPerception(ExpressPerception),
}

#[derive(Deserialize, Serialize)]
pub struct VerbDef {
    pub id: String,
    pub display_name: String,
    verb_type: VerbType,
}

#[derive(Deserialize, Serialize)]
/// Expresses a portion of an actor's perception of a target.
pub struct ExpressPerception {
    /// The actor's perception of the target's bnum_grp.
    personality: Option<[Option<BNumber>; BNUM_GROUP_SIZE]>,
    /// The actor's perception of the target's emotional state.
    emotions: Option<[Option<BNumber>; BNUM_GROUP_SIZE]>,
    /// How many of each verb the actor has perceived from the target.
    observed_verbs: Option<HashMap<String, u32>>,
}
