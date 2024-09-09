use std::fmt;
use crate::models::personality::Personality;

// Define a macro to generate the Display implementation
macro_rules! impl_personality_display {
    ($($field:ident),*) => {
        impl fmt::Display for Personality {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Personality {{ {} }}",
                    vec![$(format!("{}: {}", stringify!($field), self.$field)),*].join(", ")
                )
            }
        }
    };
}

impl_personality_display!(bad_good, false_honest, timid_dominant, ascetic_hedonistic);

#[cfg(test)]
mod tests {
    use crate::models::Personality;
    use crate::models::BNumber;
    use crate::personality;

    #[test]
    fn personality_display() {
        let personality = personality!(0.5f32, -0.25f32, -0.75f32, 0.0f32);
        assert_eq!(personality.to_string(), 
                   "Personality { bad_good: +0.500, false_honest: -0.250, \
                   timid_dominant: -0.750, ascetic_hedonistic: +0.000 }");
    }
}
