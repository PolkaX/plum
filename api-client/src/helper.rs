// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::Serialize;
use serde_json::{value::Serializer as JsonValueSerializer, Value as JsonValue};

#[inline]
pub fn serialize<T: Serialize>(value: &T) -> JsonValue {
    value
        .serialize(JsonValueSerializer)
        .expect("Types never fail to serialize")
}
