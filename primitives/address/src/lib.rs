// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The general address the represents multiple protocols.

#![deny(missing_docs)]

mod address;
mod errors;
mod network;
mod protocol;
mod serde;

/// Some constants used in this library.
mod constant {
    /// The length of a BLS signature.
    pub const BLS_SIGNATURE_LEN: usize = 96;
    /// The length of a BLS private key.
    pub const BLS_PRIVATE_KEY_LEN: usize = 32;
    /// The length of a BLS public key.
    pub const BLS_PUBLIC_KEY_LEN: usize = 48;

    /// The length of a secp256k1 signature.
    pub const SECP256K1_SIGNATURE_LEN: usize = 64;
    /// The length of a secp256k1 private key.
    pub const SECP256K1_PRIVATE_KEY_LEN: usize = 32;
    /// The length of a secp256k1 full public key.
    pub const SECP256K1_FULL_PUBLIC_KEY_LEN: usize = 65;
    /// The length of a secp256k1 raw public key.
    pub const SECP256K1_RAW_PUBLIC_KEY_LEN: usize = 64;
    /// The length of a secp256k1 compressed public key.
    pub const SECP256K1_COMPRESSED_PUBLIC_KEY_LEN: usize = 33;

    /// The max length of an address encoded as a string,
    /// which includes the network prefix, protocol, and `BLS` public key.
    pub const MAX_ADDRESS_STRING_LEN: usize = 2 + 84;
    /// The hash length taken over addresses using the `Actor` and `Secp256k1` protocols.
    pub const PAYLOAD_HASH_LEN: usize = 20;
    /// The hash length used for calculating address checksums.
    pub const CHECKSUM_HASH_LEN: usize = 4;

    /// The length of MaxUint64 as a string.
    pub(crate) const MAX_U64_LEN: usize = 20;
}

pub use self::address::{checksum, validate_checksum, Address};
pub use self::constant::*;
pub use self::errors::AddressError;
pub use self::network::{set_network, Network, NETWORK_DEFAULT};
pub use self::protocol::Protocol;
