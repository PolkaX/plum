// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! MessageReceipt, SignedMessage, and UnsignedMessage with CBOR and JSON serialization/deserialization.

#![deny(missing_docs)]

mod message_receipt;
mod signed_message;
mod unsigned_message;

pub use self::message_receipt::MessageReceipt;
pub use self::signed_message::SignedMessage;
pub use self::unsigned_message::UnsignedMessage;
