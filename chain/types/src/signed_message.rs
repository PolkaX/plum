// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{message::Message, signature::Signature};

pub struct SignedMessage {
    pub message: Message,
    pub signature: Signature,
}
