use anyhow::{anyhow, Result};
use bls::Serialize;

pub fn bls_sign(privkey: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
    let privkey = match bls::PrivateKey::from_bytes(privkey) {
        Ok(privkey) => privkey,
        Err(_) => return Err(anyhow!("BLS")),
    };
    let signature = privkey.sign(&msg);
    Ok(signature.as_bytes())
}

pub fn bls_generate_secret() -> Vec<u8> {
    let privkey = bls::PrivateKey::generate(&mut rand::rngs::OsRng);
    privkey.as_bytes()
}

pub struct BlsSigner;
