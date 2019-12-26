// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use address::keypair::{key_types, KeyPair};
use bls::Serialize;
use secp256k1::Message;

pub fn signature(pair: KeyPair, msg: &[u8]) -> Result<Vec<u8>, ()> {
    match pair.key_type {
        key_types::BLS => {
            let private_key = bls::PrivateKey::from_bytes(pair.privkey.as_slice()).unwrap();
            let signature = private_key.sign(&msg);
            Ok(signature.as_bytes())
        }
        key_types::SECP256K1 => {
            let secert = secp256k1::SecretKey::parse_slice(pair.privkey.as_slice()).unwrap();
            let mssage = Message::parse_slice(msg).unwrap();
            let (signature, _) = secp256k1::sign(&mssage, &secert);
            Ok(signature.serialize().to_vec())
        }
        _ => unreachable!(),
    }
}