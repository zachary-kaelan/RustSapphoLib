use std::fmt;
use crate::math::personality::Personality;

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
