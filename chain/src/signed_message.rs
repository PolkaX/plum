// Copyright 2019 chainnet.tech

use crate::{Message, Signature};

pub struct SignedMessage {
    message: Message,
    signature: Signature,
}
