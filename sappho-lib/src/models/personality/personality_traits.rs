use std::fmt;
use std::convert::From;
use crate::{BNumber, Personality, NUM_PERSONALITY_VALUES};
use crate::{PERSONALITY_VALUE_NAMES};


impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..NUM_PERSONALITY_VALUES {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{}: {:+.3}", PERSONALITY_VALUE_NAMES[i], self.values[i]));
        }
        write!(f, "{}", result)
    }
}

impl From<[f32; NUM_PERSONALITY_VALUES]> for Personality { 
    fn from(value: [f32; NUM_PERSONALITY_VALUES]) -> Self {
        let values = value.iter()
            .map(|x| { BNumber::new(*x) })
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Expected a Vec of length 4");
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use crate::Personality;
    use crate::BNumber;
    use crate::personality;

    #[test]
    fn personality_display() {
        let personality = personality!(0.5f32, -0.25f32, -0.75f32, 0.0f32);
        assert_eq!(personality.to_string(),
                   "bad_good: +0.500, false_honest: -0.250, \
                   timid_dominant: -0.750, ascetic_hedonistic: +0.000");
    }
}
