/// Creates a `BNumber` from a bounded f32 number, 0.0 default.
#[macro_export]
macro_rules! bnum {
    ( $n:expr ) => {
        BNumber::new($n)
    };
    ( ) => {
        BNumber::new(0.0f32)
    };
}

/// Creates a `BNumbGroup` from a set of bounded f32 numbers, 0.0 default.
#[macro_export]
macro_rules! bnum_grp {
    ( $( $x:expr ),+ ) => {
        BnumGroup {
            values: [
                $(
                    BNumber::new($x),
                )*
            ]
        }
    };
    () => {
        {
            let empty = [0.0f32; crate::consts::BNUM_GROUP_SIZE];
            BnumGroup::from(empty)
        }
    };
}

/// Creates a `BNumber` from a bounded f32 number, 0.0 default.
#[macro_export]
macro_rules! sparse_bnum {
    ( $n:expr ) => {
        SparseBNumber::new(Some($n))
    };
    ( ) => {
        SparseBNumber::new(None)
    };
}

/// Creates a `SparseBNumGroup` from a set of bounded f32 numbers, 0.0 default.
#[macro_export]
macro_rules! sparse_bnum_grp {
    ( $( $x:expr ),+ ) => {
        SparseBnumGroup {
            values: Some([
                $(
                    SparseBNumber::new(if f32::is_nan($x) { None } else { Some($x) }),
                )*
            ])
        }
    };
    () => {
        {
            SparseBnumGroup { values: None }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::consts::BNUM_GROUP_SIZE;
    use crate::BNumber;
    use crate::BnumGroup;

    #[test]
    fn bnum_macro() {
        let new_bnum1 = bnum!(0.5f32);
        assert_eq!(f32::from(new_bnum1), 0.5f32);
        let new_bnum2 = bnum!();
        assert_eq!(f32::from(new_bnum2), 0.0f32);
    }

    #[test]
    fn personality_macro() {
        let new_personality = bnum_grp!(0.5f32, 0.25f32, 0.75f32, 0.0f32);
        let expected_personality: [f32; BNUM_GROUP_SIZE] =
            [0.5f32, 0.25f32, 0.75f32, 0.0f32];
        for (value, expected) in
            new_personality.values.iter().zip(expected_personality)
        {
            assert_eq!(f32::from(*value), expected);
        }
    }
}
