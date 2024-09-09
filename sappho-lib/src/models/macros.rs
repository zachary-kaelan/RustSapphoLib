/// Creates a `BNumber` from a bounded number, 0.0 default.
#[macro_export]
macro_rules! bnum {
    ( $n:expr ) => ( BNumber::new($n) );
    ( ) => ( BNumber::new(0.0f32) );
}

#[macro_export]
macro_rules! personality {
        ($bad_good:expr, $false_honest:expr, $timid_dominant:expr, $ascetic_hedonistic:expr) => {
        Personality::new(
            BNumber::new($bad_good),
            BNumber::new($false_honest),
            BNumber::new($timid_dominant),
            BNumber::new($ascetic_hedonistic))
    };
    () => {
        Personality::new(
            BNumber::new(0.0f32),
            BNumber::new(0.0f32),
            BNumber::new(0.0f32),
            BNumber::new(0.0f32))
    }
}

#[cfg(test)]
mod tests {
    use crate::BNumber;
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
        assert_eq!(f32::from(new_personality.bad_good), 0.5f32);
        assert_eq!(f32::from(new_personality.false_honest), 0.25f32);
        assert_eq!(f32::from(new_personality.timid_dominant), 0.75f32);
        assert_eq!(f32::from(new_personality.ascetic_hedonistic), 0.0f32);
    }
}