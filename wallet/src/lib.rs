// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

extern crate bls_signatures as bls;

mod error;
mod keystore;
mod wallet;

pub use self::error::{Result, WalletError};
pub use self::keystore::{KeyInfo, KeyStore, KeyType, MemKeyStore, DEFAULT_KEYSTORE_PATH};
pub use self::wallet::{generate_key, Wallet};

#[test]
fn test_wallet() {
    let keystore = MemKeyStore::new();
    let mut wallet = Wallet::new(keystore);
    let addr = wallet.generate_key(KeyType::Secp256k1).unwrap();
    assert!(wallet.has_key(&addr));
}
