// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use plum_message::{MessageReceipt, UnsignedMessage};

///
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecutionResult {
    ///
    pub msg: UnsignedMessage,
    ///
    pub msg_rct: MessageReceipt,
    ///
    pub error: String,
    ///
    pub subcalls: Vec<ExecutionResult>,
}
