use crate::BNumber;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::From;
use std::fmt::{Display, Formatter};

impl Display for BNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+.3}", self.val)
    }
}

impl From<BNumber> for f32 {
    fn from(value: BNumber) -> Self {
        value.val
    }
}

impl Serialize for BNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.val)
    }
}

impl<'de> Deserialize<'de> for BNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BNumberVisitor;

        impl<'de> Visitor<'de> for BNumberVisitor {
            type Value = f32;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "A float between -1.0 and 1.0 (non-inclusive)")
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if ((-1f32 + f32::EPSILON)..=(1f32 - f32::EPSILON)).contains(&v) {
                    Ok(v)
                } else {
                    Err(Error::custom("Value outside of range"))
                }
            }
        }

        let val = deserializer.deserialize_f32(BNumberVisitor)?;
        Ok(BNumber::new(val))
    }
}
