use std::collections::BTreeSet;

use bitfield_rle::{decode, encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct BitField(BTreeSet<u64>);

impl Serialize for BitField {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, S::Error>
    where
        S: Serializer,
    {
        // self.0.
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for BitField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        unimplemented!()
    }
}
