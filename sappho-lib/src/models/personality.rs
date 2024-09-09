mod personality_math;
mod personality_traits;


use crate::BNumber;

#[derive(Clone, Copy)]
pub struct Personality {
    pub bad_good: BNumber,
    pub false_honest: BNumber,
    pub timid_dominant: BNumber,
    pub ascetic_hedonistic: BNumber
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
