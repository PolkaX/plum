// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{block_header::BlockHeader, message::Message, signed_message::SignedMessage};

pub struct FullBlock {
    pub header: BlockHeader,
    pub bls_messages: Vec<Message>,
    pub secp256k1_messages: Vec<SignedMessage>,
}
