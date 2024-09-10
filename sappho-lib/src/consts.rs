use crate::BNumber;

pub const BNUM_GROUP_SIZE: usize = 4;
pub type PersonalityT = [BNumber; BNUM_GROUP_SIZE];

pub type BnumGroupT = [BNumber; BNUM_GROUP_SIZE];

pub const NUM_VERB_VALUES: usize = 4;

pub type VerbT = [BNumber; BNUM_GROUP_SIZE];