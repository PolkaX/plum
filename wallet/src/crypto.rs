use blake2_rfc::blake2b::blake2b;
use data_encoding::Specification;
use std::convert::TryFrom;

// PayloadHashLength defines the hash length taken over addresses using the Actor and SECP256K1 protocols.
pub const PAYLOAD_HASH_LENGTH: usize = 20;

// ChecksumHashLength defines the hash length used for calculating address checksums.
pub const CHECKSUM_HASH_LENGTH: usize = 4;

pub const ENCODE_STD: &str = "abcdefghijklmnopqrstuvwxyz234567";

pub fn blake2b_hash(ingest: &[u8], hash_config: usize) -> Vec<u8> {
    let hash = blake2b(hash_config, &[], ingest);
    hash.as_bytes().to_vec()
}

pub fn address_hash(ingest: &[u8]) -> Vec<u8> {
    blake2b_hash(ingest, PAYLOAD_HASH_LENGTH)
}

pub fn checksum(ingest: &[u8]) -> Vec<u8> {
    blake2b_hash(ingest, CHECKSUM_HASH_LENGTH)
}

pub fn base32_encode(input: &[u8]) -> String {
    let mut spec = Specification::new();
    spec.symbols.push_str(ENCODE_STD);
    spec.padding = None;
    let encoder = spec.encoding().unwrap();

    encoder.encode(&input)
}

/// An identifier for a type of cryptographic key.
///
/// To avoid clashes with other modules when distributing your module publically, register your
/// `KeyTypeId` on the list here by making a PR.
///
/// Values whose first character is `_` are reserved for private use and won't conflict with any
/// public modules.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyTypeId(pub [u8; 4]);

impl From<u32> for KeyTypeId {
    fn from(x: u32) -> Self {
        Self(x.to_le_bytes())
    }
}

impl From<KeyTypeId> for u32 {
    fn from(x: KeyTypeId) -> Self {
        u32::from_le_bytes(x.0)
    }
}

impl<'a> TryFrom<&'a str> for KeyTypeId {
    type Error = ();
    fn try_from(x: &'a str) -> Result<Self, ()> {
        let b = x.as_bytes();
        if b.len() != 4 {
            return Err(());
        }
        let mut res = KeyTypeId::default();
        res.0.copy_from_slice(&b[0..4]);
        Ok(res)
    }
}

/// Known key types; this also functions as a global registry of key types for projects wishing to
/// avoid collisions with each other.
///
/// It's not universal in the sense that *all* key types need to be mentioned here, it's just a
/// handy place to put common key types.
pub mod key_types {
    use super::KeyTypeId;
    /// Key type for Babe module, build-in.
    pub const BLS: KeyTypeId = KeyTypeId(*b"bls0");
    /// Key type for Grandpa module, build-in.
    pub const SECP256K1: KeyTypeId = KeyTypeId(*b"secp");
}

#[test]
fn address_hash_should_work() {
    let ingest = [115, 97, 116, 111, 115, 104, 105];
    let hashed = [
        71, 22, 176, 35, 183, 254, 132, 182, 231, 220, 218, 48, 60, 61, 117, 75, 26, 143, 242, 252,
    ];
    assert_eq!(address_hash(&ingest[..]), hashed.to_vec());
}

#[test]
fn base32_encoding_should_work() {
    let input = [
        253, 29, 15, 77, 252, 215, 233, 154, 252, 185, 154, 131, 38, 183, 220, 69, 157, 50, 198,
        40, 148, 236, 248, 227,
    ];
    let base32_encoded = "7uoq6tp427uzv7fztkbsnn64iwotfrristwpryy";
    assert_eq!(base32_encoded.to_string(), base32_encode(&input));
}
