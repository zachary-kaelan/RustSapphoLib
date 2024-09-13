use crate::{BNumber, SparseBNumber};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::From;
use std::fmt::{Display, Formatter};

impl Display for SparseBNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.bnum {
            None => {
                write!(f, "None")
            }
            Some(bnum) => bnum.fmt(f),
        }
    }
}

impl From<SparseBNumber> for Option<f32> {
    fn from(value: SparseBNumber) -> Self {
        value.bnum.map(f32::from)
    }
}

impl Serialize for SparseBNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.bnum {
            None => serializer.serialize_none(),
            Some(bnum) => bnum.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for SparseBNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SparseBNumberVisitor;

        impl<'de> Visitor<'de> for SparseBNumberVisitor {
            type Value = Option<f32>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "A float between -1.0 and 1.0 (non-inclusive), or None"
                )
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if ((-1f32 + f32::EPSILON)..=(1f32 - f32::EPSILON)).contains(&v) {
                    Ok(Some(v))
                } else {
                    Err(Error::custom("Value outside of range"))
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }
        }

        let val = deserializer.deserialize_option(SparseBNumberVisitor)?;
        Ok(SparseBNumber::new(val))
    }
}
