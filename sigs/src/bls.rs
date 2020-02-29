// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::{anyhow, Result};
use bls::Serialize;

/// Sign the given `msg` with the `bls` key that corresponds to the given private key.
///
/// Returns the signature.
pub fn bls_sign(privkey: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
    let privkey = match bls::PrivateKey::from_bytes(privkey) {
        Ok(privkey) => privkey,
        Err(_) => return Err(anyhow!("failed to generate BLS PrivateKey from slice")),
    };
    let signature = privkey.sign(&msg);
    Ok(signature.as_bytes())
}

/// Generate a `bls` private key.
///
/// Returns the private key.
pub fn bls_generate_secret() -> Vec<u8> {
    let privkey = bls::PrivateKey::generate(&mut rand::rngs::OsRng);
    privkey.as_bytes()
}
