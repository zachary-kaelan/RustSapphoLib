use crate::consts::BNUM_GROUP_SIZE;
use crate::value_aliases::PERSONALITY_VALUE_ALIASES;
use crate::{SparseBNumber, SparseBnumGroup};
use std::convert::From;
use std::fmt;

impl fmt::Display for SparseBnumGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.values {
            None => {
                write!(f, "None")
            }
            Some(values) => {
                let mut result = String::new();
                let mut has_value = false;
                for i in 0..BNUM_GROUP_SIZE {
                    let value: Option<f32> = Option::from(values[i]);
                    match value {
                        None => {}
                        Some(value) => {
                            if has_value {
                                result.push_str(", ");
                            }
                            result.push_str(&format!(
                                "{}: {:+.3}",
                                PERSONALITY_VALUE_ALIASES[i], value
                            ));
                            has_value = true;
                        }
                    }
                }
                write!(f, "{}", result)
            }
        }
    }
}

impl From<[f32; BNUM_GROUP_SIZE]> for SparseBnumGroup {
    fn from(value: [f32; BNUM_GROUP_SIZE]) -> Self {
        let values = value
            .iter()
            .map(|x| SparseBNumber::new(Some(*x)))
            .collect::<Vec<SparseBNumber>>()
            .try_into()
            .expect("Expected a Vec of length 4");
        Self {
            values: Some(values),
        }
    }
}

impl From<[Option<f32>; BNUM_GROUP_SIZE]> for SparseBnumGroup {
    fn from(value: [Option<f32>; BNUM_GROUP_SIZE]) -> Self {
        let values = value
            .iter()
            .map(|x| SparseBNumber::new(*x))
            .collect::<Vec<SparseBNumber>>()
            .try_into()
            .expect("Expected a Vec of length 4");
        Self {
            values: Some(values),
        }
    }
}

impl From<[SparseBNumber; BNUM_GROUP_SIZE]> for SparseBnumGroup {
    fn from(value: [SparseBNumber; BNUM_GROUP_SIZE]) -> Self {
        Self::new(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::sparse_bnum_grp;
    use crate::{SparseBNumber, SparseBnumGroup};

    #[test]
    fn personality_display() {
        let personality = sparse_bnum_grp!(0.5f32, -0.25f32, -0.75f32, 0.0f32);
        assert_eq!(
            personality.to_string(),
            "bad_good: +0.500, faithless_honest: -0.250, \
                   timid_dominant: -0.750, ascetic_hedonistic: +0.000"
        );
    }

    #[test]
    fn sparse_personality_display() {
        let personality = sparse_bnum_grp!(f32::NAN, -0.25f32, f32::NAN, 0.0f32);
        assert_eq!(
            personality.to_string(),
            "faithless_honest: -0.250, \
                   ascetic_hedonistic: +0.000"
        );
    }
}
