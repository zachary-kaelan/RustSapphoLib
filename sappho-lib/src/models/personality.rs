mod personality_math;
mod personality_traits;


use crate::models::bnum::BNumber;

#[derive(Clone, Copy)]
struct Personality {
    bad_good: BNumber,
    false_honest: BNumber,
    timid_dominant: BNumber,
    ascetic_hedonistic: BNumber
}

// Define a macro to generate the new function
macro_rules! impl_personality_new {
    ($($field:ident),*) => {
        impl Personality {
            pub fn new($($field: BNumber),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}

impl_personality_new!(bad_good, false_honest, timid_dominant, ascetic_hedonistic);
