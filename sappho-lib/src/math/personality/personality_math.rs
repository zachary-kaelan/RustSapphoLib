use std::ops::{Add, Sub};
use crate::math::personality::Personality;

// Define a macro to generate the blend_with implementation
macro_rules! impl_personality_blend_with {
    ($($field:ident),*) => {
        impl Personality {
            /// Blends the bounded values of `self` and `other` based on weight `pos`.
            /// e.g. a `pos` of 0.0 returns `self` and a `pos` of 1.0 returns `other`
            fn blend_with(self, other: Self, pos: f32) -> Self {
                Self {
                    $($field: self.$field.blend_with(other.$field, pos)),*
                }
            }
        }
    };
}

// Define a macro to generate the Add implementation
macro_rules! impl_personality_add {
    ($($field:ident),*) => {
        impl Add for Personality {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field),*
                }
            }
        }
    };
}

// Define a macro to generate the Sub implementation
macro_rules! impl_personality_sub {
    ($($field:ident),*) => {
        impl Sub for Personality {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field),*
                }
            }
        }
    };
}

impl_personality_blend_with!(bad_good, false_honest, timid_dominant, ascetic_hedonistic);
impl_personality_add!(bad_good, false_honest, timid_dominant, ascetic_hedonistic);
impl_personality_sub!(bad_good, false_honest, timid_dominant, ascetic_hedonistic);