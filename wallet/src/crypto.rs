use blake2_rfc::blake2b::{blake2b, Blake2b};
use data_encoding::{Specification, BASE32};

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
