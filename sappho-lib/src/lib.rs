mod models;
mod comps;

pub use crate::models::BNumber;
pub use crate::models::Personality;
const NUM_PERSONALITY_VALUES: usize = 4;
type PersonalityT = [BNumber; NUM_PERSONALITY_VALUES];
const PERSONALITY_VALUE_NAMES: [&str; NUM_PERSONALITY_VALUES] = ["bad_good", "false_honest", "timid_dominant", "ascetic_hedonistic"];

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
