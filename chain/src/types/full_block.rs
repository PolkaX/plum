// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::types::{BlockHeader, Message, SignedMessage};

pub struct FullBlock {
    header: BlockHeader,
    bls_messages: Vec<Message>,
    secpk_messages: Vec<SignedMessage>,
}
