// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::{message::Message, signature::Signature};

pub struct SignedMessage {
    pub message: Message,
    pub signature: Signature,
}
