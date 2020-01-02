// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;

use blake2_rfc::blake2b::blake2b;
use integer_encoding::VarInt;
use serde::{Deserialize, Serialize};

use crate::constant;
use crate::error::{AddressError, Result};

/// The length of MaxUint64 as a string.
const MAX_U64_LEN: usize = 20;

/// Protocol Identifier.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Protocol {
    /// ID protocol, identifier: 0.
    ID,
    /// SECP256K1 protocol, identifier: 1.
    SECP256K1,
    /// Actor protocol, identifier: 2.
    Actor,
    /// BLS protocol, identifier: 3.
    BLS,
}

impl TryFrom<u8> for Protocol {
    type Error = AddressError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Protocol::ID),
            1 => Ok(Protocol::SECP256K1),
            2 => Ok(Protocol::Actor),
            3 => Ok(Protocol::BLS),
            _ => Err(AddressError::UnknownProtocol),
        }
    }
}

/// The network type used by the address.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Network {
    /// Main network, prefix: 'f'.
    Main,
    /// Test network, prefix: 't'.
    Test,
}

impl Network {
    /// Return the prefix identifier of network.
    pub fn prefix(self) -> char {
        match self {
            Network::Main => 'f',
            Network::Test => 't',
        }
    }
}

/// The general address structure.
#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Address {
    // ID protocol: payload is VarInt encoding.
    // SECP256K1 protocol: payload is pubkey (length = 20)
    // Actor protocol: payload length = 20
    // BLS protocol: payload is pubkey (length = 48)
    bytes: Vec<u8>, // bytes = protocol (1 byte) + payload
}

impl Address {
    /// Create an address using the ID protocol.
    pub fn new_id_addr(id: u64) -> Result<Self> {
        let mut payload = [0u8; MAX_U64_LEN];
        id.encode_var(&mut payload);
        let mut bytes = Vec::with_capacity(1 + payload.len());
        bytes.push(Protocol::ID as u8);
        bytes.extend_from_slice(&payload);
        Ok(Address { bytes })
    }

    /// Create an address using the SECP256k1 protocol.
    pub fn new_secp256k1_addr(pubkey: &[u8]) -> Result<Self> {
        let payload = address_hash(pubkey);
        if payload.len() != constant::PAYLOAD_HASH_LEN {
            return Err(AddressError::InvalidPayload);
        }
        let mut bytes = Vec::with_capacity(1 + constant::PAYLOAD_HASH_LEN);
        bytes.push(Protocol::SECP256K1 as u8);
        bytes.extend_from_slice(&payload);
        Ok(Address { bytes })
    }

    /// Create an address using the Actor protocol.
    pub fn new_actor_addr(data: &[u8]) -> Result<Self> {
        let payload = address_hash(data);
        if payload.len() != constant::PAYLOAD_HASH_LEN {
            return Err(AddressError::InvalidPayload);
        }
        let mut bytes = Vec::with_capacity(1 + constant::PAYLOAD_HASH_LEN);
        bytes.push(Protocol::Actor as u8);
        bytes.extend_from_slice(&payload);
        Ok(Address { bytes })
    }

    /// Create an address using the BLS protocol.
    pub fn new_bls_addr(pubkey: &[u8]) -> Result<Self> {
        let payload = pubkey;
        if payload.len() != constant::BLS_PUBLIC_KEY_LEN {
            return Err(AddressError::InvalidPayload);
        }
        let mut bytes = Vec::with_capacity(1 + constant::BLS_PUBLIC_KEY_LEN);
        bytes.push(Protocol::BLS as u8);
        bytes.extend_from_slice(payload);
        Ok(Address { bytes })
    }

    /// Create an address represented by the bytes `addr` (protocol + payload).
    pub fn new_from_bytes(addr: &[u8]) -> Result<Self> {
        if addr.len() <= 1 {
            return Err(AddressError::InvalidLength);
        }
        match Protocol::try_from(addr[0])? {
            Protocol::ID => {
                let (id, len) = u64::decode_var(&addr[1..]);
                if len != (&addr[1..]).len() {
                    return Err(AddressError::InvalidPayload);
                }
                Self::new_id_addr(id)
            }
            Protocol::SECP256K1 => Self::new_secp256k1_addr(&addr[1..]),
            Protocol::Actor => Self::new_actor_addr(&addr[1..]),
            Protocol::BLS => Self::new_bls_addr(&addr[1..]),
        }
    }

    /// Return the protocol of the address.
    pub fn protocol(&self) -> Protocol {
        Protocol::try_from(self.bytes[0]).expect("Converting u8 into Protocol will never fail")
    }

    /// Return the payload of the address.
    pub fn payload(&self) -> &[u8] {
        &self.bytes[1..]
    }

    /// Return the address (protocol + payload) as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the checksum of (protocol + payload).
    pub fn checksum(&self) -> Vec<u8> {
        checksum(&self.bytes)
    }

    /// Return an address encoded as a string.
    pub fn encode(&self, network: Network) -> String {
        match self.protocol() {
            Protocol::SECP256K1 | Protocol::Actor | Protocol::BLS => {
                let mut payload_and_checksum = self.payload().to_vec();
                payload_and_checksum.extend_from_slice(&self.checksum());
                let base32 = base32_encode(payload_and_checksum);
                format!("{}{}{}", network.prefix(), self.protocol() as u8, base32)
            }
            Protocol::ID => {
                let (id, _) = u64::decode_var(&self.payload());
                format!("{}{}{}", network.prefix(), self.protocol() as u8, id)
            }
        }
    }

    /// Decode the addr string into the address.
    pub fn decode(addr: &str) -> Result<Address> {
        if addr.len() < 3 || addr.len() > constant::MAX_ADDRESS_STRING_LEN {
            return Err(AddressError::InvalidLength);
        }

        let network = addr.chars().nth(0).expect("The length of addr >= 3");
        if network != Network::Main.prefix() && network != Network::Test.prefix() {
            return Err(AddressError::UnknownNetwork);
        }

        let protocol = match addr.chars().nth(1).expect("The length of addr >= 3") {
            '0' => Protocol::ID,
            '1' => Protocol::SECP256K1,
            '2' => Protocol::Actor,
            '3' => Protocol::BLS,
            _ => return Err(AddressError::UnknownProtocol),
        };

        let raw_addr = &addr[2..];
        match protocol {
            Protocol::ID => {
                if raw_addr.len() > MAX_U64_LEN {
                    return Err(AddressError::InvalidLength);
                }
                match raw_addr.parse::<u64>() {
                    Ok(id) => Self::new_id_addr(id),
                    Err(_) => Err(AddressError::InvalidPayload),
                }
            }
            Protocol::SECP256K1 => Self::new_with_check(
                Protocol::SECP256K1,
                raw_addr.as_bytes(),
                constant::PAYLOAD_HASH_LEN,
            ),
            Protocol::Actor => Self::new_with_check(
                Protocol::Actor,
                raw_addr.as_bytes(),
                constant::PAYLOAD_HASH_LEN,
            ),
            Protocol::BLS => Self::new_with_check(
                Protocol::BLS,
                raw_addr.as_bytes(),
                constant::BLS_PUBLIC_KEY_LEN,
            ),
        }
    }

    // A helper function for `decode` function.
    fn new_with_check(protocol: Protocol, raw_addr: &[u8], payload_size: usize) -> Result<Self> {
        let decoded = base32_decode(raw_addr)?;
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
        Ok(Address { bytes })
    }
}

/// Validate whether the checksum of `ingest` is equal to `expect`.
pub fn validate_checksum(ingest: &[u8], expect: &[u8]) -> bool {
    let digest = checksum(ingest);
    digest.as_slice() == expect
}

/// Return the checksum of ingest.
pub fn checksum(ingest: &[u8]) -> Vec<u8> {
    blake2b_hash(ingest, constant::CHECKSUM_HASH_LEN)
}

pub(crate) fn address_hash(ingest: &[u8]) -> Vec<u8> {
    blake2b_hash(ingest, constant::PAYLOAD_HASH_LEN)
}

fn blake2b_hash(ingest: &[u8], hash_config: usize) -> Vec<u8> {
    let hash = blake2b(hash_config, &[], ingest);
    hash.as_bytes().to_vec()
}

fn base32_encode(input: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32_NOPAD
        .encode(input.as_ref())
        .to_ascii_lowercase()
}

fn base32_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    Ok(data_encoding::BASE32_NOPAD.decode(&input.as_ref().to_ascii_uppercase())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let addr = Address::decode("t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a").unwrap();
        let checksum = addr.checksum();
        assert!(validate_checksum(addr.as_bytes(), checksum.as_slice()));
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
