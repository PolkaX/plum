// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod error;
mod keystore;
mod wallet;

pub use self::error::{Result, WalletError};
pub use self::keystore::{KeyStore, SignKeyInfo};
pub use self::wallet::Wallet;
