use crate::consts::BNUM_GROUP_SIZE;
use crate::scripting::BnumType;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::iter::zip;
use std::sync::{LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(PartialEq, Clone, Debug)]
pub enum AliasType {
    Personality,
    Perception,
    SelfPerception,
    Accordance,
    Verb,
}

pub struct Aliases {
    personality: RwLock<HashMap<String, usize>>,
    perception: RwLock<HashMap<String, usize>>,
    self_perception: RwLock<HashMap<String, usize>>,
    accordance: RwLock<HashMap<String, usize>>,
    verb: RwLock<HashMap<String, usize>>,
}

impl Aliases {
    fn new() -> Self {
        Self {
            personality: RwLock::new(HashMap::new()),
            perception: RwLock::new(HashMap::new()),
            self_perception: RwLock::new(HashMap::new()),
            accordance: RwLock::new(HashMap::new()),
            verb: RwLock::new(HashMap::new()),
        }
    }

    fn get_alias_dict(
        &self,
        alias_type: &AliasType,
    ) -> LockResult<RwLockReadGuard<HashMap<String, usize>>> {
        match alias_type {
            AliasType::Personality => self.personality.read(),
            AliasType::Perception => self.perception.read(),
            AliasType::SelfPerception => self.self_perception.read(),
            AliasType::Accordance => self.accordance.read(),
            AliasType::Verb => self.verb.read(),
        }
    }

    fn get_alias_dict_mut(
        &self,
        alias_type: &AliasType,
    ) -> LockResult<RwLockWriteGuard<HashMap<String, usize>>> {
        match alias_type {
            AliasType::Personality => self.personality.write(),
            AliasType::Perception => self.perception.write(),
            AliasType::SelfPerception => self.self_perception.write(),
            AliasType::Accordance => self.accordance.write(),
            AliasType::Verb => self.verb.write(),
        }
    }

    pub fn add_alias(alias_type: &AliasType, alias: &str, index: usize) {
        let alias_dict = ALIASES.get_alias_dict_mut(alias_type);

        alias_dict.unwrap().insert(String::from(alias), index);
    }

    pub fn add_alias_for_existing(
        alias_type: &AliasType,
        alias1: &str,
        alias2: &str,
    ) -> Option<usize> {
        let mut alias_dict = ALIASES.get_alias_dict_mut(alias_type).unwrap();
        let existing_index = *{ alias_dict.get(alias1).or(alias_dict.get(alias2)) }?;
        alias_dict.insert(String::from(alias2), existing_index);
        Some(existing_index)
    }

    pub fn get_index_for_alias(alias_type: &AliasType, alias: &str) -> Option<usize> {
        let mut alias_dict = ALIASES.get_alias_dict(alias_type).unwrap();
        let mut index = alias_dict.get(alias);
        if index.is_none()
            && (*alias_type == AliasType::SelfPerception || *alias_type == AliasType::Accordance)
        {
            alias_dict = ALIASES.get_alias_dict(&AliasType::Perception).unwrap();
            index = alias_dict.get(alias);
        }
        if index.is_none() && *alias_type != AliasType::Personality {
            alias_dict = ALIASES.get_alias_dict(&AliasType::Personality).unwrap();
            index = alias_dict.get(alias);
        }
        index.copied()
    }

    pub fn get_index_for_bnum_alias(bnum_type: &BnumType, alias: &str) -> Option<usize> {
        let alias_type = match bnum_type {
            BnumType::Personality => AliasType::Personality,
            BnumType::Accordance => AliasType::Accordance,
            BnumType::SelfPerception => AliasType::SelfPerception,
            BnumType::Perception => AliasType::Perception,
            BnumType::Circumstantial => AliasType::Perception,
        };

        Self::get_index_for_alias(&alias_type, alias)
    }

    pub fn init() {
        // Initialize defaults
        for (alias_type, aliases) in zip(
            [
                AliasType::Personality,
                AliasType::Perception,
                AliasType::Accordance,
                AliasType::Verb,
            ],
            [
                PERSONALITY_VALUE_ALIASES,
                PERCEPTION_VALUE_ALIASES,
                ACCORDANCE_VALUE_ALIASES,
                VERB_VALUE_ALIASES,
            ],
        ) {
            for (i, alias) in aliases.iter().enumerate() {
                Self::add_alias(&alias_type, alias, i);
            }
        }
        Self::add_alias_for_existing(&AliasType::Personality, "False_Honest", "Faithless_Honest");
    }
}

static ALIASES: Lazy<Aliases> = Lazy::new(Aliases::new);

const PERSONALITY_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "bad_good",
    "faithless_honest",
    "timid_dominant",
    "ascetic_hedonistic",
];
const PERCEPTION_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "dislike_affection",
    "mistrust_faith",
    "disregard_heed",
    "respect_disgust",
];
const ACCORDANCE_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "misanthropy_altruism",
    "paranoia_gullibility",
    "control_helplessness",
    "austerity_decadence",
];
const VERB_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "selfish_altruistic",
    "deceptive_sincere",
    "submissive_dominant",
    "cold_emotional",
];
