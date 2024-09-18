use crate::consts::BNUM_GROUP_SIZE;
use crate::{BNumber, BnumGroup};
use std::convert::From;
use std::fmt;

impl fmt::Display for BnumGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..BNUM_GROUP_SIZE {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:+.3}", self.values[i]));
        }
        write!(f, "{}", result)
    }
}

impl From<[f32; BNUM_GROUP_SIZE]> for BnumGroup {
    fn from(value: [f32; BNUM_GROUP_SIZE]) -> Self {
        let values = value
            .iter()
            .map(|x| BNumber::new(*x))
            .collect::<Vec<BNumber>>()
            .try_into()
            .expect("Expected a Vec of length 4");
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use crate::bnum_grp;
    use crate::BNumber;
    use crate::BnumGroup;

    #[test]
    fn personality_display() {
        let personality = bnum_grp!(0.5f32, -0.25f32, -0.75f32, 0.0f32);
        assert_eq!(
            personality.to_string(),
            "bad_good: +0.500, faithless_honest: -0.250, \
                   timid_dominant: -0.750, ascetic_hedonistic: +0.000"
        );
    }
}
