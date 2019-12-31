// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::{KeyPair, KeyType};
use bls::Serialize;
use secp256k1::Message;

pub fn signature(pair: KeyPair, msg: &[u8]) -> Result<Vec<u8>, ()> {
    match pair.key_type {
        KeyType::BLS => {
            let private_key = bls::PrivateKey::from_bytes(pair.privkey.as_slice()).unwrap();
            let signature = private_key.sign(&msg);
            Ok(signature.as_bytes())
        }
        KeyType::SECP256K1 => {
            let seckey = secp256k1::SecretKey::parse_slice(pair.privkey.as_slice()).unwrap();
            let message = Message::parse_slice(msg).unwrap();
            let (signature, _) = secp256k1::sign(&message, &seckey);
            Ok(signature.serialize().to_vec())
        }
        _ => unreachable!(),
    }
}
