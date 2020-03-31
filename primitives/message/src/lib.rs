// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! MessageReceipt, SignedMessage, and UnsignedMessage with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod message_receipt;
mod signed_message;
mod unsigned_message;

pub use self::message_receipt::{
    cbor as message_receipt_cbor, json as message_receipt_json, MessageReceipt,
};
pub use self::signed_message::{
    cbor as signed_message_cbor, json as signed_message_json, SignedMessage,
};
pub use self::unsigned_message::{
    cbor as unsigned_message_cbor, json as unsigned_message_json, UnsignedMessage,
};
