use std::collections::HashMap;
use crate::BNumber;
use crate::consts::BNUM_GROUP_SIZE;

/// Expresses a portion of an actor's perception of a target. 
pub struct ExpressPerception {
    /// The actor's perception of the target's bnum_grp.
    personality: Option<[Option<BNumber>; BNUM_GROUP_SIZE]>,
    /// The actor's perception of the target's emotional state.
    emotions: Option<[Option<BNumber>; BNUM_GROUP_SIZE]>,
    /// How many of each verb the actor has perceived from the target.
    observed_verbs: Option<HashMap<String, u32>>
}