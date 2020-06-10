// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::str::FromStr;

use plum_hashing::blake2b_variable;

use crate::constant;
use crate::errors::AddressError;
use crate::network::{Network, NETWORK_DEFAULT, NETWORK_MAINNET_PREFIX, NETWORK_TESTNET_PREFIX};
use crate::protocol::Protocol;

/// The general address structure.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Address {
    // `ID` protocol: payload is VarInt encoding.
    // `Secp256k1` protocol: payload is the hash of pubkey (length = 20)
    // `Actor` protocol: payload length = 20
    // `BLS` protocol: payload is pubkey (length = 48)
    protocol: Protocol,
    payload: Vec<u8>,
}

impl Address {
    /// Create an address with the given protocol and payload
    pub(crate) fn new<T: Into<Vec<u8>>>(
        protocol: Protocol,
        payload: T,
    ) -> Result<Self, AddressError> {
        let payload = payload.into();
        match protocol {
            Protocol::Id => {}
            Protocol::Secp256k1 | Protocol::Actor => {
                if payload.len() != constant::PAYLOAD_HASH_LEN {
                    return Err(AddressError::InvalidPayload);
                }
            }
            Protocol::Bls => {
                if payload.len() != constant::BLS_PUBLIC_KEY_LEN {
                    return Err(AddressError::InvalidPayload);
                }
            }
        }

        Ok(Self { protocol, payload })
    }

    /// Create an address using the `Id` protocol.
    pub fn new_id_addr(id: u64) -> Result<Self, AddressError> {
        let mut payload_buf = unsigned_varint::encode::u64_buffer();
        let payload = unsigned_varint::encode::u64(id, &mut payload_buf);
        Self::new(Protocol::Id, payload)
    }

    /// Create an address using the `Secp256k1` protocol.
    pub fn new_secp256k1_addr(pubkey: &[u8]) -> Result<Self, AddressError> {
        if pubkey.len() != constant::SECP256K1_FULL_PUBLIC_KEY_LEN
            && pubkey.len() != constant::SECP256K1_RAW_PUBLIC_KEY_LEN
            && pubkey.len() != constant::SECP256K1_COMPRESSED_PUBLIC_KEY_LEN
        {
            return Err(AddressError::InvalidPayload);
        }
        Self::new(Protocol::Secp256k1, address_hash(pubkey))
    }

    /// Create an address using the `Actor` protocol.
    pub fn new_actor_addr(data: &[u8]) -> Result<Self, AddressError> {
        Self::new(Protocol::Actor, address_hash(data))
    }

    /// Create an address using the `BLS` protocol.
    pub fn new_bls_addr(pubkey: &[u8]) -> Result<Self, AddressError> {
        Self::new(Protocol::Bls, pubkey)
    }

    /// Create an address represented by the encoding bytes `addr` (protocol + payload).
    pub fn new_from_bytes(addr: &[u8]) -> Result<Self, AddressError> {
        if addr.len() <= 1 {
            return Err(AddressError::InvalidLength);
        }
        let protocol = Protocol::try_from(addr[0])?;
        Self::new(protocol, &addr[1..])
    }

    /// Return the network type of the address.
    pub fn network(&self) -> Network {
        **NETWORK_DEFAULT
    }

    /// Return the protocol of the address.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Return the payload of the address.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// If the `Address` is an ID address, return the ID of Address if possible.
    /// Returns None otherwise.
    pub fn as_id(&self) -> Option<u64> {
        if let Protocol::Id = self.protocol {
            let id = unsigned_varint::decode::u64(&self.payload)
                .expect("unsigned varint decode payload of ID Address shouldn't be fail; qed")
                .0;
            Some(id)
        } else {
            None
        }
    }

    /// Return the encoded bytes of address (protocol + payload).
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.payload.len());
        bytes.push(self.protocol as u8);
        bytes.extend_from_slice(self.payload());
        bytes
    }

    /// Return the checksum of (protocol + payload).
    pub fn checksum(&self) -> Vec<u8> {
        checksum(&self.as_bytes())
    }

    // A helper function for `from_str`.
    fn new_with_check(
        protocol: Protocol,
        raw: &[u8],
        payload_size: usize,
    ) -> Result<Self, AddressError> {
        let decoded = base32_decode(raw)?;
        let (payload, checksum) = decoded.split_at(decoded.len() - constant::CHECKSUM_HASH_LEN);
        if payload.len() != payload_size {
            return Err(AddressError::InvalidPayload);
        }

        let mut bytes = Vec::with_capacity(1 + payload_size);
        bytes.push(protocol as u8);
        bytes.extend_from_slice(payload);
        if !validate_checksum(&bytes, checksum) {
            return Err(AddressError::InvalidChecksum);
        }

        Ok(Self {
            protocol,
            payload: payload.to_vec(),
        })
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.protocol() {
            Protocol::Id => {
                let id = unsigned_varint::decode::u64(self.payload())
                    .expect("unsigned varint decode shouldn't be fail")
                    .0;
                write!(
                    f,
                    "{}{}{}",
                    NETWORK_DEFAULT.prefix(),
                    self.protocol() as u8,
                    id
                )
            }
            Protocol::Secp256k1 | Protocol::Actor | Protocol::Bls => {
                let mut payload_and_checksum = self.payload().to_vec();
                payload_and_checksum.extend_from_slice(&checksum(&self.as_bytes()));
                let base32 = base32_encode(payload_and_checksum);
                write!(
                    f,
                    "{}{}{}",
                    NETWORK_DEFAULT.prefix(),
                    self.protocol() as u8,
                    base32
                )
            }
        }
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || s.len() > constant::MAX_ADDRESS_STRING_LEN {
            return Err(AddressError::InvalidLength);
        }

        match &s[0..1] {
            NETWORK_MAINNET_PREFIX | NETWORK_TESTNET_PREFIX => {
                if &s[0..1] != NETWORK_DEFAULT.prefix() {
                    return Err(AddressError::MismatchNetwork);
                }
            }
            _ => return Err(AddressError::UnknownNetwork),
        }

        let protocol = match &s[1..2] {
            "0" => Protocol::Id,
            "1" => Protocol::Secp256k1,
            "2" => Protocol::Actor,
            "3" => Protocol::Bls,
            _ => return Err(AddressError::UnknownProtocol),
        };

        let raw = &s[2..];

        match protocol {
            Protocol::Id => {
                if raw.len() > constant::MAX_U64_LEN {
                    return Err(AddressError::InvalidLength);
                }
                match raw.parse::<u64>() {
                    Ok(id) => Self::new_id_addr(id),
                    Err(_) => Err(AddressError::InvalidPayload),
                }
            }
            Protocol::Secp256k1 => Self::new_with_check(
                Protocol::Secp256k1,
                raw.as_bytes(),
                constant::PAYLOAD_HASH_LEN,
            ),
            Protocol::Actor => {
                Self::new_with_check(Protocol::Actor, raw.as_bytes(), constant::PAYLOAD_HASH_LEN)
            }
            Protocol::Bls => {
                Self::new_with_check(Protocol::Bls, raw.as_bytes(), constant::BLS_PUBLIC_KEY_LEN)
            }
        }
    }
}

/// Validate whether the checksum of `ingest` is equal to `expect`.
pub fn validate_checksum(ingest: &[u8], expect: &[u8]) -> bool {
    let digest = checksum(ingest);
    digest.as_slice() == expect
}

/// Return the checksum of ingest.
pub fn checksum(ingest: &[u8]) -> Vec<u8> {
    blake2b_variable(ingest, constant::CHECKSUM_HASH_LEN)
}

fn address_hash(ingest: &[u8]) -> Vec<u8> {
    blake2b_variable(ingest, constant::PAYLOAD_HASH_LEN)
}

fn base32_encode(input: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_NOPAD
        .encode(input.as_ref())
        .to_ascii_lowercase()
}

fn base32_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, AddressError> {
    Ok(data_encoding::BASE32_NOPAD.decode(&input.as_ref().to_ascii_uppercase())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_payload() {
        let id_addr = Address::new_id_addr(12_512_063u64).unwrap();
        assert_eq!(id_addr.payload(), [191, 214, 251, 5]);
    }

    #[test]
    fn test_checksum() {
        unsafe { crate::set_network(Network::Test) };
        let addr = Address::from_str("t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a").unwrap();
        let checksum = addr.checksum();
        assert!(validate_checksum(&addr.as_bytes(), checksum.as_slice()));
    }

    #[test]
    fn test_address_hash() {
        let ingest = [115, 97, 116, 111, 115, 104, 105];
        let hashed = [
            71, 22, 176, 35, 183, 254, 132, 182, 231, 220, 218, 48, 60, 61, 117, 75, 26, 143, 242,
            252,
        ];
        assert_eq!(address_hash(&ingest[..]), hashed.to_vec());
    }

    #[test]
    fn test_base32_codec() {
        let input = [
            253, 29, 15, 77, 252, 215, 233, 154, 252, 185, 154, 131, 38, 183, 220, 69, 157, 50,
            198, 40, 148, 236, 248, 227,
        ];
        let encoded = base32_encode(input);
        assert_eq!(encoded, "7uoq6tp427uzv7fztkbsnn64iwotfrristwpryy");
        let decoded = base32_decode(encoded).unwrap();
        assert_eq!(decoded, input);
    }
}
