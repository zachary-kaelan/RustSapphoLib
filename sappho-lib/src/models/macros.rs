use crate::{NUM_PERSONALITY_VALUES, PERSONALITY_VALUE_NAMES};

/// Creates a `BNumber` from a bounded f32 number, 0.0 default.
#[macro_export]
macro_rules! bnum {
    ( $n:expr ) => ( BNumber::new($n) );
    ( ) => ( BNumber::new(0.0f32) );
}

/// Creates a `Personality` from a set of bounded f32 numbers, 0.0 default.
#[macro_export]
macro_rules! personality {
    ( $( $x:expr ),+ ) => {
        Personality {
            values: [
                $(
                    BNumber::new($x),
                )*
            ]
        }
    };
    () => {
        {
            let empty = [0.0f32; crate::NUM_PERSONALITY_VALUES];
            Personality::from(empty)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{BNumber, NUM_PERSONALITY_VALUES};
    use crate::Personality;

    #[test]
    fn bnum_macro() {
        let new_bnum1 = bnum!(0.5f32);
        assert_eq!(f32::from(new_bnum1), 0.5f32);
        let new_bnum2 = bnum!();
        assert_eq!(f32::from(new_bnum2), 0.0f32);
    }

    #[test]
    fn personality_macro() {
        let new_personality = personality!(0.5f32, 0.25f32, 0.75f32, 0.0f32);
        let expected_personality: [f32; NUM_PERSONALITY_VALUES] = [0.5f32, 0.25f32, 0.75f32, 0.0f32];
        for (value, expected) in new_personality.values.iter().zip(expected_personality) {
            assert_eq!(f32::from(*value), expected);
        }
    }
}