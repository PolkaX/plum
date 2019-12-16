// Copyright 2019 PolkaX. Licensed under GPL-3.0.

use crate::types::{Message, Signature};

pub struct SignedMessage {
    message: Message,
    signature: Signature,
}
