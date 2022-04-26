#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

// From https://github.com/serde-rs/serde/issues/984#issuecomment-314143738
pub(crate) fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}