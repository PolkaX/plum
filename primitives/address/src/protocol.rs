// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::convert;
use std::fmt;

use crate::errors::AddressError;

/// Protocol Identifier.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Protocol {
    /// ID protocol, identifier: 0.
    Id = 0,
    /// Secp256k1 protocol, identifier: 1.
    Secp256k1 = 1,
    /// Actor protocol, identifier: 2.
    Actor = 2,
    /// BLS protocol, identifier: 3.
    Bls = 3,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::Id
    }
}

impl convert::TryFrom<u8> for Protocol {
    type Error = AddressError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Protocol::Id),
            1 => Ok(Protocol::Secp256k1),
            2 => Ok(Protocol::Actor),
            3 => Ok(Protocol::Bls),
            _ => Err(AddressError::UnknownProtocol),
        }
    }
}

impl From<Protocol> for u8 {
    fn from(protocol: Protocol) -> Self {
        match protocol {
            Protocol::Id => 0,
            Protocol::Secp256k1 => 1,
            Protocol::Actor => 2,
            Protocol::Bls => 3,
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}
