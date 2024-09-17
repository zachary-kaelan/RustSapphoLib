use crate::consts::BNUM_GROUP_SIZE;
use crate::scripting::BnumType;

pub enum ValueAliasType {
    Personality,
    Perception,
    Accordance,
    Verb
}

pub fn get_value_aliases_from_type(alias_type: ValueAliasType) -> [&'static str; BNUM_GROUP_SIZE] {
    match alias_type {
        ValueAliasType::Personality => { PERSONALITY_VALUE_ALIASES }
        ValueAliasType::Perception => { PERCEPTION_VALUE_ALIASES }
        ValueAliasType::Accordance => { ACCORDANCE_VALUE_ALIASES }
        ValueAliasType::Verb => { VERB_VALUE_ALIASES }
    }
}

pub fn get_value_aliases_from_bnum_type(bnum_type: &BnumType) -> [&'static str; BNUM_GROUP_SIZE] {
    match bnum_type {
        BnumType::Personality => { PERSONALITY_VALUE_ALIASES }
        BnumType::Accordance => { ACCORDANCE_VALUE_ALIASES }
        BnumType::Perception => { PERCEPTION_VALUE_ALIASES }
        BnumType::Circumstantial => { PERCEPTION_VALUE_ALIASES }
        BnumType::SelfPerception => { PERCEPTION_VALUE_ALIASES }
    }
}

pub const PERSONALITY_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "bad_good",
    "faithless_honest",
    "timid_dominant",
    "ascetic_hedonistic",
];
pub const PERCEPTION_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "dislike_affection",
    "mistrust_faith",
    "disregard_heed",
    "respect_disgust",
];
pub const ACCORDANCE_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "misanthropy_altruism",
    "paranoia_gullibility",
    "control_helplessness",
    "austerity_decadence",
];
pub const VERB_VALUE_ALIASES: [&str; BNUM_GROUP_SIZE] = [
    "selfish_altruistic",
    "deceptive_sincere",
    "submissive_dominant",
    "cold_emotional",
];
