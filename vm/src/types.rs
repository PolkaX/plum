// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use plum_message::{MessageReceipt, UnsignedMessage};

///
#[doc(hidden)]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecutionResult {
    pub msg: UnsignedMessage,
    pub msg_rct: MessageReceipt,
    pub error: String,
    pub duration: i64, // time.Duration is a alias of i64 in golang
    pub subcalls: Vec<ExecutionResult>,
}
