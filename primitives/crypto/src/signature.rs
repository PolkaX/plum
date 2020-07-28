// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_address::{Address, Protocol};
use plum_hashing::blake2b_256;

use crate::errors::CryptoError;

/// The signature type.
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureType {
    /// The `Secp256k1` signature.
    Secp256k1 = 1,
    /// The `BLS` signature.
    Bls = 2,
}

impl Default for SignatureType {
    fn default() -> Self {
        SignatureType::Bls
    }
}

impl TryFrom<u8> for SignatureType {
    type Error = CryptoError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SignatureType::Secp256k1),
            2 => Ok(SignatureType::Bls),
            _ => Err(CryptoError::UnknownSignatureType(value)),
        }
    }
}

impl From<SignatureType> for u8 {
    fn from(ty: SignatureType) -> Self {
        match ty {
            SignatureType::Secp256k1 => 1,
            SignatureType::Bls => 2,
        }
    }
}

/// The general signature structure.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signature {
    /// The signature type.
    r#type: SignatureType,
    /// Tha actual signature bytes.
    /// secp256k1: signature (64 bytes) + recovery_id (1 byte)
    /// bls: signature (96 bytes)
    #[serde(with = "plum_bytes::base64")]
    data: Vec<u8>,
}

impl Signature {
    /// Create a signature with the given type and raw data
    pub fn new<T: Into<Vec<u8>>>(ty: SignatureType, data: T) -> Self {
        Self {
            r#type: ty,
            data: data.into(),
        }
    }

    /// Create a Secp256k1 signature with the given raw data.
    pub fn new_secp256k1<T: Into<Vec<u8>>>(data: T) -> Self {
        Self::new(SignatureType::Secp256k1, data)
    }

    /// Create a `BLS` signature with the given raw data.
    pub fn new_bls<T: Into<Vec<u8>>>(data: T) -> Self {
        Self::new(SignatureType::Bls, data)
    }

    /// Sign the message with the given signature type and private key.
    ///
    /// Return the signature related to the given signature type.
    pub fn sign<K, M>(ty: SignatureType, privkey: K, msg: M) -> Result<Self, CryptoError>
    where
        K: AsRef<[u8]>,
        M: AsRef<[u8]>,
    {
        match ty {
            SignatureType::Secp256k1 => Self::sign_secp256k1(privkey, msg),
            SignatureType::Bls => Self::sign_bls(privkey, msg),
        }
    }

    /// Sign the message with the given secp256k1 private key.
    ///
    /// Return the secp256k1 signature.
    pub fn sign_secp256k1<K, M>(privkey: K, msg: M) -> Result<Self, CryptoError>
    where
        K: AsRef<[u8]>,
        M: AsRef<[u8]>,
    {
        let seckey = secp256k1::SecretKey::parse_slice(privkey.as_ref())?;
        let hashed_msg = blake2b_256(msg);
        let message = secp256k1::Message::parse(&hashed_msg);
        let (signature, recovery_id) = secp256k1::sign(&message, &seckey);
        let mut data = Vec::with_capacity(secp256k1::util::SIGNATURE_SIZE + 1);
        data.extend_from_slice(&signature.serialize());
        data.push(recovery_id.serialize());
        Ok(Self {
            r#type: SignatureType::Secp256k1,
            data,
        })
    }

    /// Sign the message with the given `BLS` private key.
    ///
    /// Return the `BLS` signature.
    pub fn sign_bls<K, M>(privkey: K, msg: M) -> Result<Self, CryptoError>
    where
        K: AsRef<[u8]>,
        M: AsRef<[u8]>,
    {
        use bls::Serialize;
        let privkey = bls::PrivateKey::from_bytes(privkey.as_ref())?;
        let signature = privkey.sign(msg);
        Ok(Self {
            r#type: SignatureType::Bls,
            data: signature.as_bytes(),
        })
    }

    /// Verify the signature with the given address and message.
    pub fn verify<M: AsRef<[u8]>>(&self, addr: &Address, msg: M) -> Result<bool, CryptoError> {
        let protocol = addr.protocol();
        match (self.r#type, protocol) {
            (SignatureType::Secp256k1, Protocol::Secp256k1) => {
                let hashed_msg = blake2b_256(msg);
                let message = secp256k1::Message::parse(&hashed_msg);
                assert_eq!(self.data.len(), secp256k1::util::SIGNATURE_SIZE + 1);
                let mut signature = [0u8; secp256k1::util::SIGNATURE_SIZE];
                signature.copy_from_slice(&self.data[..secp256k1::util::SIGNATURE_SIZE]);
                let signature = secp256k1::Signature::parse(&signature);
                let recovery_id = self.data[secp256k1::util::SIGNATURE_SIZE];
                let recovery_id = secp256k1::RecoveryId::parse(recovery_id)?;
                let pubkey = secp256k1::recover(&message, &signature, &recovery_id)?;

                let recovery_addr = Address::new_secp256k1_addr(&pubkey.serialize()[..])
                    .expect("secp256k1 pubkey must be valid; qed");
                Ok(&recovery_addr == addr)
            }
            (SignatureType::Bls, Protocol::Bls) => Ok(self.verify_bls(addr.payload(), msg)?),
            _ => Err(CryptoError::NotSameType(self.r#type, protocol)),
        }
    }

    /// Verify the signature with the given public key and message.
    pub fn verify_raw<K, M>(&self, pubkey: K, msg: M) -> Result<bool, CryptoError>
    where
        K: AsRef<[u8]>,
        M: AsRef<[u8]>,
    {
        match self.r#type {
            SignatureType::Secp256k1 => self.verify_secp256k1(pubkey, msg),
            SignatureType::Bls => self.verify_bls(pubkey, msg),
        }
    }

    /// Verify the secp256k1 signature with the given secp256k1 public key and message.
    fn verify_secp256k1<K, M>(&self, pubkey: K, msg: M) -> Result<bool, CryptoError>
    where
        K: AsRef<[u8]>,
        M: AsRef<[u8]>,
    {
        let hashed_msg = blake2b_256(msg);
        let message = secp256k1::Message::parse(&hashed_msg);
        assert_eq!(self.data.len(), secp256k1::util::SIGNATURE_SIZE + 1);
        let signature = &self.data[..secp256k1::util::SIGNATURE_SIZE];
        let signature = secp256k1::Signature::parse_slice(&signature)?;
        let pubkey = secp256k1::PublicKey::parse_slice(pubkey.as_ref(), None)?;
        Ok(secp256k1::verify(&message, &signature, &pubkey))
    }

    /// Verify the `BLS` signature with the given `BLS` public key and message.
    fn verify_bls<K, M>(&self, pubkey: K, msg: M) -> Result<bool, CryptoError>
    where
        K: AsRef<[u8]>,
        M: AsRef<[u8]>,
    {
        use bls::Serialize;
        let pubkey = bls::PublicKey::from_bytes(pubkey.as_ref())?;
        // When signing with `BLS` privkey, the message will be hashed in `bls::PrivateKey::sign`,
        // so the message here needs to be hashed before the signature is verified.
        let hashed_msg = bls::hash(msg.as_ref());
        let signature = bls::Signature::from_bytes(&self.data)?;
        Ok(bls::verify(&signature, &[hashed_msg], &[pubkey]))
    }

    /// Return the signature type.
    pub fn r#type(&self) -> SignatureType {
        self.r#type
    }

    /// Return the actual signature bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }
}

// Implement CBOR serialization for Signature.
impl encode::Encode for Signature {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        let mut bytes = Vec::with_capacity(self.data.len() + 1);
        bytes.push(u8::from(self.r#type));
        bytes.extend_from_slice(&self.data);
        e.bytes(&bytes)?.ok()
    }
}

// Implement CBOR deserialization for Signature.
impl<'b> decode::Decode<'b> for Signature {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let bytes = d.bytes()?;
        let r#type = SignatureType::try_from(bytes[0])
            .map_err(|_| decode::Error::Message("expected signature type"))?;
        Ok(Signature {
            r#type,
            data: (&bytes[1..]).to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Signature, SignatureType};
    use crate::key::{PrivateKey, PublicKey};

    #[test]
    fn sign_and_verify_secp256k1() {
        let privkey = PrivateKey::generate_secp256k1_privkey();
        let pubkey = PublicKey::from_privkey(&privkey);
        let (privkey, pubkey) = (privkey.into_vec(), pubkey.into_vec());
        let msg = "hello, world";
        let signature = Signature::sign_secp256k1(privkey, msg).unwrap();
        let res = signature.verify_secp256k1(&pubkey, msg);
        assert_eq!(res, Ok(true));

        let addr = Address::new_secp256k1_addr(&pubkey).unwrap();
        let res = signature.verify(&addr, msg);
        assert_eq!(res, Ok(true));
    }

    #[test]
    fn sign_and_verify_bls() {
        let privkey = PrivateKey::generate_bls_privkey();
        let pubkey = PublicKey::from_privkey(&privkey);
        let (privkey, pubkey) = (privkey.into_vec(), pubkey.into_vec());
        let msg = "hello, world";
        let signature = Signature::sign_bls(privkey, msg).unwrap();
        let res = signature.verify_bls(&pubkey, msg);
        assert_eq!(res, Ok(true));

        let addr = Address::new_bls_addr(&pubkey).unwrap();
        let res = signature.verify(&addr, msg);
        assert_eq!(res, Ok(true));
    }

    #[test]
    fn signature_cbor_serde() {
        let cases = vec![(
            Signature {
                r#type: SignatureType::Bls,
                data: b"boo! im a signature".to_vec(),
            },
            vec![
                84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32, 115, 105, 103, 110, 97, 116,
                117, 114, 101,
            ],
        )];

        for (signature, expected) in cases {
            let ser = minicbor::to_vec(&signature).unwrap();
            assert_eq!(ser, expected);
            let de = minicbor::decode::<Signature>(&ser).unwrap();
            assert_eq!(signature, de);
        }
    }

    #[test]
    fn signature_json_serde() {
        let cases = vec![(
            Signature {
                r#type: SignatureType::Bls,
                data: b"boo! im a signature".to_vec(),
            },
            r#"{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="}"#,
        )];

        for (signature, expected) in cases {
            let ser = serde_json::to_string(&signature).unwrap();
            assert_eq!(ser, expected);
            let de = serde_json::from_str::<Signature>(&ser).unwrap();
            assert_eq!(signature, de);
        }
    }
}
