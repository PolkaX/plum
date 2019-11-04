// Copyright 2019 chainnet.tech

use crate::types::{Message, Signature};

pub struct SignedMessage {
    message: Message,
    signature: Signature,
}
