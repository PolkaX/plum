use crate::signer::Signer;
use anyhow::Result;
use plum_hashing::blake2b_variable;

pub struct Secp256k1Signer;

pub fn secp256k1_sign(privkey: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
    let seckey = secp256k1::SecretKey::parse_slice(privkey)?;
    let blake2b_hash = blake2b_variable(msg, secp256k1::util::MESSAGE_SIZE);
    let message = secp256k1::Message::parse_slice(&blake2b_hash)?;
    let (signature, _) = secp256k1::sign(&message, &seckey);
    Ok(signature.serialize().to_vec())
}

pub fn secp256k1_generate_secret() -> Vec<u8> {
    let seckey = secp256k1::SecretKey::random(&mut rand::rngs::OsRng);
    seckey.serialize().to_vec()
}
