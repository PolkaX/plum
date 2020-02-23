// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use plum_hashing::blake2b_variable;

use crate::constant;
use crate::error::AddressError;
use crate::network::{Network, NETWORK_DEFAULT, NETWORK_MAINNET_PREFIX, NETWORK_TESTNET_PREFIX};
use crate::protocol::Protocol;

/// The general address structure.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Address {
    network: Network,
    // ID protocol: payload is VarInt encoding.
    // SECP256K1 protocol: payload is pubkey (length = 20)
    // Actor protocol: payload length = 20
    // BLS protocol: payload is pubkey (length = 48)
    protocol: Protocol,
    payload: Vec<u8>,
}

impl Address {
    /// Create an address with the given network, protocol and payload
    fn new<T: Into<Vec<u8>>>(
        network: Network,
        protocol: Protocol,
        payload: T,
    ) -> Result<Self, AddressError> {
        let payload = payload.into();
        match protocol {
            Protocol::ID => {}
            Protocol::SECP256K1 | Protocol::Actor => {
                if payload.len() != constant::PAYLOAD_HASH_LEN {
                    return Err(AddressError::InvalidPayload);
                }
            }
            Protocol::BLS => {
                if payload.len() != constant::BLS_PUBLIC_KEY_LEN {
                    return Err(AddressError::InvalidPayload);
                }
            }
        }

        Ok(Self {
            network,
            protocol,
            payload,
        })
    }

    /// Create an address using the ID protocol.
    pub fn new_id_addr(network: Network, id: u64) -> Result<Self, AddressError> {
        let mut payload_buf = unsigned_varint::encode::u64_buffer();
        let payload = unsigned_varint::encode::u64(id, &mut payload_buf);
        Self::new(network, Protocol::ID, payload)
    }

    /// Create an address using the SECP256k1 protocol.
    pub fn new_secp256k1_addr(network: Network, pubkey: &[u8]) -> Result<Self, AddressError> {
        Self::new(network, Protocol::SECP256K1, address_hash(pubkey))
    }

    /// Create an address using the Actor protocol.
    pub fn new_actor_addr(network: Network, data: &[u8]) -> Result<Self, AddressError> {
        Self::new(network, Protocol::Actor, address_hash(data))
    }

    /// Create an address using the BLS protocol.
    pub fn new_bls_addr(network: Network, pubkey: &[u8]) -> Result<Self, AddressError> {
        Self::new(network, Protocol::BLS, pubkey)
    }

    /// Create an address represented by the encoding bytes `addr` (protocol + payload).
    pub fn new_from_bytes(network: Network, addr: &[u8]) -> Result<Self, AddressError> {
        if addr.len() <= 1 {
            return Err(AddressError::InvalidLength);
        }
        let protocol = Protocol::try_from(addr[0])?;
        Self::new(network, protocol, &addr[1..])
    }

    /// Return the network type of the address.
    pub fn network(&self) -> Network {
        self.network
    }

    /// Return the protocol of the address.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Return the payload of the address.
    pub fn payload(&self) -> &[u8] {
        &self.payload
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
        network: Network,
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
            network,
            protocol,
            payload: payload.to_vec(),
        })
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.protocol() {
            Protocol::ID => {
                let id = unsigned_varint::decode::u64(self.payload())
                    .expect("unsigned varint decode shouldn't be fail")
                    .0;
                write!(
                    f,
                    "{}{}{}",
                    self.network().prefix(),
                    self.protocol() as u8,
                    id
                )
            }
            Protocol::SECP256K1 | Protocol::Actor | Protocol::BLS => {
                let mut payload_and_checksum = self.payload().to_vec();
                payload_and_checksum.extend_from_slice(&checksum(&self.as_bytes()));
                let base32 = base32_encode(payload_and_checksum);
                write!(
                    f,
                    "{}{}{}",
                    self.network().prefix(),
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

        let network = match &s[0..1] {
            NETWORK_MAINNET_PREFIX => Network::Main,
            NETWORK_TESTNET_PREFIX => Network::Test,
            _ => return Err(AddressError::UnknownNetwork),
        };

        let protocol = match &s[1..2] {
            "0" => Protocol::ID,
            "1" => Protocol::SECP256K1,
            "2" => Protocol::Actor,
            "3" => Protocol::BLS,
            _ => return Err(AddressError::UnknownProtocol),
        };

        let raw = &s[2..];

        match protocol {
            Protocol::ID => {
                if raw.len() > constant::MAX_U64_LEN {
                    return Err(AddressError::InvalidLength);
                }
                match raw.parse::<u64>() {
                    Ok(id) => Self::new_id_addr(network, id),
                    Err(_) => Err(AddressError::InvalidPayload),
                }
            }
            Protocol::SECP256K1 => Self::new_with_check(
                network,
                Protocol::SECP256K1,
                raw.as_bytes(),
                constant::PAYLOAD_HASH_LEN,
            ),
            Protocol::Actor => Self::new_with_check(
                network,
                Protocol::Actor,
                raw.as_bytes(),
                constant::PAYLOAD_HASH_LEN,
            ),
            Protocol::BLS => Self::new_with_check(
                network,
                Protocol::BLS,
                raw.as_bytes(),
                constant::BLS_PUBLIC_KEY_LEN,
            ),
        }
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.as_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: serde_bytes::ByteBuf = Deserialize::deserialize(deserializer)?;
        let mut bytes = bytes.into_vec();
        let protocol = Protocol::try_from(bytes.remove(0)).map_err(serde::de::Error::custom)?;
        Ok(Self::new(NETWORK_DEFAULT, protocol, bytes).map_err(serde::de::Error::custom)?)
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
        let id_addr = Address::new_id_addr(Network::Test, 12512063u64).unwrap();
        assert_eq!(id_addr.payload(), [191, 214, 251, 5]);
    }

    #[test]
    fn test_address_serde() {
        let id_addr = Address::new_id_addr(Network::Test, 12512063u64).unwrap();
        let ser = serde_cbor::to_vec(&id_addr).unwrap();
        assert_eq!(ser, [69, 0, 191, 214, 251, 5]);
        let de = serde_cbor::from_slice(&ser).unwrap();
        assert_eq!(id_addr, de);
    }

    #[test]
    fn test_checksum() {
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
