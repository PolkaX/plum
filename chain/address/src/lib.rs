// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The general address the represents multiple protocols.

#![deny(missing_docs)]

mod address;
mod error;

pub use self::address::{checksum, validate_checksum, Address, Network, Protocol};
pub use self::error::{AddressError, Result};

/// Some constants used in this library.
pub mod constant {
    /// The length of a BLS signature.
    pub const BLS_SIGNATURE_LEN: usize = 96;
    /// The length of a BLS private key.
    pub const BLS_PRIVATE_KEY_LEN: usize = 32;
    /// The length of a BLS public key.
    pub const BLS_PUBLIC_KEY_LEN: usize = 48;
    /// The length of a BLS message hash/digest.
    pub const BLS_DIGEST_LEN: usize = 96;

    /// The max length of an address encoded as a string,
    /// which includes the network prefix, protocol, and bls public key.
    pub const MAX_ADDRESS_STRING_LEN: usize = 2 + 84;
    /// The hash length taken over addresses using the Actor and SECP256K1 protocols.
    pub const PAYLOAD_HASH_LEN: usize = 20;
    /// The hash length used for calculating address checksums.
    pub const CHECKSUM_HASH_LEN: usize = 4;
}
