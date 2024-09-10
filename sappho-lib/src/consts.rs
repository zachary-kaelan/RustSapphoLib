use crate::BNumber;

pub const BNUM_GROUP_SIZE: usize = 4;
pub type PersonalityT = [BNumber; BNUM_GROUP_SIZE];

pub type BnumGroupT = [BNumber; BNUM_GROUP_SIZE];

pub const PERSONALITY_VALUE_NAMES: [&str; BNUM_GROUP_SIZE] = ["bad_good", "faithless_honest", "timid_dominant", "ascetic_hedonistic"];
pub const NUM_VERB_VALUES: usize = 4;

pub type VerbT = [BNumber; BNUM_GROUP_SIZE];

pub const VERB_VALUE_NAMES: [&str; BNUM_GROUP_SIZE] = ["selfish_altruistic", "deceptive_sincere", "submissive_dominant", "cold_emotional"];
pub const VERB_WEIGHTS: [[f32; BNUM_GROUP_SIZE]; BNUM_GROUP_SIZE] = [
//  s_a      d_s      s_d      c_e
    [ 0.550,  0.100,  0.000,  0.100], // bad_good
    [ 0.200,  0.750,  0.000,  0.250], // faithless_honest
    [ 0.000,  0.000,  0.850,  0.000], // timid_dominant
    [-0.250, -0.150,  0.150,  0.650], // ascetic_hedonistic
];

/*
Rationale:
  selfish_altruistic:
    bad_good (primary): generally we associate altruism with being good.
    faithless_honest: someone with more integrity is more likely to engage in altruism, even if just for mutual benefit.
    ascetic_hedonistic: hedonism can often come at the expense of others (e.g., sadistic villains).
  deceptive_sincere:
    bad_good: sincerity isn't always good, but deception can often be used for nastiness.
    faithless_honest (primary)
    ascetic_hedonistic: deception tends to be more fun.
  submissive_dominant:
    timid_dominant (primary)
    ascetic_hedonistic: being powerful tends to be more fun.
  cold_emotional:
    bad_good: being too cold is more likely to be seen as evil than being too emotional.
    faithless_honest: expressing oneself emotionally can be a component of honesty.
    ascetic_hedonistic (primary): hedonism is in pursuit of strong emotions.
 */