// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_bigint::BigInt;

/// The receipt of applying message.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MessageReceipt {
    /// The exit code of VM.
    pub exit_code: u8,
    /// The return bytes.
    pub ret: Vec<u8>,
    /// The used number of gas.
    pub gas_used: BigInt,
}
