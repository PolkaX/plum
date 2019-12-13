use blake2_rfc::blake2b::{blake2b, Blake2b};
use data_encoding::{Specification, BASE32};

use std::convert::TryFrom;

pub mod secp256k1;

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    // Unknown address network
    UnknownNetwork,
    // Unknown address protocol
    UnknownProtocol,
    // Invalid address payload
    InvalidPayload,
    // Invalid address length
    InvalidLength,
    // Invalid address checksum
    InvalidChecksum,
}

// UndefAddressString is the string used to represent an empty address when encoded to a string.
// const UndefAddressString: '&static str = "<empty>";

// MaxAddressStringLength is the max length of an address encoded as a string
// it include the network prefx, protocol, and bls publickey
const MaxAddressStringLength: u8 = 2 + 84;

const encodeStd: &'static str = "abcdefghijklmnopqrstuvwxyz234567";

// AddressEncoding defines the base32 config used for address encoding and decoding.
// var AddressEncoding = base32.NewEncoding(encodeStd)

#[derive(PartialEq, Eq, Clone)]
pub enum Network {
    Mainnet,
    Testnet,
}

// PayloadHashLength defines the hash length taken over addresses using the Actor and SECP256K1 protocols.
pub const PAYLOAD_HASH_LENGTH: usize = 20;

// ChecksumHashLength defines the hash length used for calculating address checksums.
pub const CHECKSUM_HASH_LENGTH: usize = 4;

pub const BLS_PUBLICKEY_BYTES: usize = 48;

#[derive(Debug, Clone)]
pub enum Address {
    Id(Vec<u8>),
    Secp256k1([u8; PAYLOAD_HASH_LENGTH + 1]),
    Actor([u8; PAYLOAD_HASH_LENGTH + 1]),
    Bls(Vec<u8>),
}

impl Address {
    pub fn protocol(&self) -> u8 {
        match *self {
            Self::Id(_) => 0u8,
            Self::Secp256k1(_) => 1u8,
            Self::Actor(_) => 2u8,
            Self::Bls(_) => 3u8,
        }
    }

    pub fn payload(&self) -> Vec<u8> {
        let c = self.clone();
        match c {
            Self::Id(x) => x[1..].to_vec(),
            Self::Secp256k1(x) => x[1..].to_vec(),
            Self::Actor(x) => x[1..].to_vec(),
            Self::Bls(x) => x[1..].to_vec(),
        }
    }

    pub fn checksum(&self) -> Vec<u8> {
        match *self {
            Self::Secp256k1(addr) => checksum(&addr[..]),
            _ => Vec::new(),
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        let network = "t";
        match self {
            Self::Secp256k1(_) | Self::Actor(_) | Self::Bls(_) => {
                let payload = self.payload();
                let chsm = self.checksum();

                let mut t = Vec::new();
                t.extend_from_slice(&payload);
                t.extend_from_slice(&chsm);

                let protocol = self.protocol();

                format!("{}{}{}", network, protocol, base32_encode(&t))
            }
            Self::Id(_) => "".into(),
        }
    }
}

pub fn hash(ingest: &[u8], hash_config: usize) -> Vec<u8> {
    let hash = blake2b(hash_config, &[], ingest);
    hash.as_bytes().to_vec()
}

pub fn address_hash(ingest: &[u8]) -> Vec<u8> {
    hash(ingest, PAYLOAD_HASH_LENGTH)
}

pub fn checksum(ingest: &[u8]) -> Vec<u8> {
    hash(ingest, CHECKSUM_HASH_LENGTH)
}

const ENCODE_STD: &str = "abcdefghijklmnopqrstuvwxyz234567";

pub fn base32_encode(input: &[u8]) -> String {
    let mut spec = Specification::new();
    spec.symbols.push_str(ENCODE_STD);
    spec.padding = None;
    let encoder = spec.encoding().unwrap();

    encoder.encode(&input)
}

// cbor decode

// cbor encode

#[cfg(test)]
mod tests {
    use super::*;

    use data_encoding::HEXUPPER;
    use std::convert::TryInto;

    #[test]
    fn address_hash_should_work() {
        let ingest = [115, 97, 116, 111, 115, 104, 105];
        let hashed = [
            71, 22, 176, 35, 183, 254, 132, 182, 231, 220, 218, 48, 60, 61, 117, 75, 26, 143, 242,
            252,
        ];
        assert_eq!(address_hash(&ingest[..]), hashed.to_vec());
    }

    #[test]
    fn base32_encoding_should_work() {
        let input = [
            253, 29, 15, 77, 252, 215, 233, 154, 252, 185, 154, 131, 38, 183, 220, 69, 157, 50,
            198, 40, 148, 236, 248, 227,
        ];
        let base32_encoded = "7uoq6tp427uzv7fztkbsnn64iwotfrristwpryy";
        assert_eq!(base32_encoded.to_string(), base32_encode(&input));
    }

    #[test]
    fn new_secp256k1_address_should_work() {
        let test_cases = [
            (
                [
                    4, 148, 2, 250, 195, 126, 100, 50, 164, 22, 163, 160, 202, 84, 38, 181, 24, 90,
                    179, 178, 79, 97, 52, 239, 162, 92, 228, 135, 200, 45, 46, 78, 19, 191, 69, 37,
                    17, 224, 210, 36, 84, 33, 248, 97, 59, 193, 13, 114, 250, 33, 102, 102, 169,
                    108, 59, 193, 57, 32, 211, 255, 35, 63, 208, 188, 5,
                ],
                "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
            ),
            (
                [
                    4, 118, 135, 185, 16, 55, 155, 242, 140, 190, 58, 234, 103, 75, 18, 0, 12, 107,
                    125, 186, 70, 255, 192, 95, 108, 148, 254, 42, 34, 187, 204, 38, 2, 255, 127,
                    92, 118, 242, 28, 165, 93, 54, 149, 145, 82, 176, 225, 232, 135, 145, 124, 57,
                    53, 118, 238, 240, 147, 246, 30, 189, 58, 208, 111, 127, 218,
                ],
                "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
            ),
            (
                [
                    4, 222, 253, 208, 16, 1, 239, 184, 110, 1, 222, 213, 206, 52, 248, 71, 167, 58,
                    20, 129, 158, 230, 65, 188, 182, 11, 185, 41, 147, 89, 111, 5, 220, 45, 96, 95,
                    41, 133, 248, 209, 37, 129, 45, 172, 65, 99, 163, 150, 52, 155, 35, 193, 28,
                    194, 255, 53, 157, 229, 75, 226, 135, 234, 98, 49, 155,
                ],
                "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
            ),
            (
                [
                    4, 3, 237, 18, 200, 20, 182, 177, 13, 46, 224, 157, 149, 180, 104, 141, 178,
                    209, 128, 208, 169, 163, 122, 107, 106, 125, 182, 61, 41, 129, 30, 233, 115, 4,
                    121, 216, 239, 145, 57, 233, 18, 73, 202, 189, 57, 50, 145, 207, 229, 210, 119,
                    186, 118, 222, 69, 227, 224, 133, 163, 118, 129, 191, 54, 69, 210,
                ],
                "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
            ),
            (
                [
                    4, 247, 150, 129, 154, 142, 39, 22, 49, 175, 124, 24, 151, 151, 181, 69, 214,
                    2, 37, 147, 97, 71, 230, 1, 14, 101, 98, 179, 206, 158, 254, 139, 16, 20, 65,
                    97, 169, 30, 208, 180, 236, 137, 8, 0, 37, 63, 166, 252, 32, 172, 144, 251,
                    241, 251, 242, 113, 48, 164, 236, 195, 228, 3, 183, 5, 118,
                ],
                "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
            ),
            (
                [
                    4, 66, 131, 43, 248, 124, 206, 158, 163, 69, 185, 3, 80, 222, 125, 52, 149,
                    133, 156, 164, 73, 5, 156, 94, 136, 221, 231, 66, 133, 223, 251, 158, 192, 30,
                    186, 188, 95, 200, 98, 104, 207, 234, 235, 167, 174, 5, 191, 184, 214, 142,
                    183, 90, 82, 104, 120, 44, 248, 111, 200, 112, 43, 239, 138, 31, 224,
                ],
                "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
            ),
        ];

        for (b, s) in test_cases.into_iter() {
            let addr: crate::Address = crate::secp256k1::Public(b.to_vec()).try_into().unwrap();
            assert_eq!(s.to_string(), addr.to_string());
        }
    }
}
