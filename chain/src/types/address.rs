// Copyright 2019 PolkaX. Licensed under GPL-3.0.

lazy_static! {
    static ref BASE32: data_encoding::Encoding = {
        let mut spec = data_encoding::Specification::new();
        spec.symbols.push_str("abcdefghijklmnopqrstuvwxyz234567");
        spec.encoding().unwrap()
    };
}

const PAYLOAD_HASH_LENGTH: u32 = 20;
const CHECKSUM_HASH_LENGTH: u32 = 4;
const MAX_ADDRESS_STRING_LENGTH: u32 = 2 + 84;

pub type AddressEncoding = BASE32;

#[derive(Default, PartialEq, Clone, Eq)]
pub struct Address(Vec<u8>);

pub type Network = u8;

const MAINNET: Network = 0;
const TESTNET: Network = 1;

const MAINNET_PREFIX: u8 = b'f';

const TESTNET_PREFIX: u8 = b't';

pub type Protocol = u8;

const ID: Protocol = 0;
const SECP256K1: Protocol = 1;
const ACTOR: Protocol = 2;
const BLS: Protocol = 3;
const UNKNOWN: Protocol = 255;

/*
fn encode(network: Network, addr Address) -> (string, error) {
}
*/

impl Address {
    pub fn protocol(&self) -> Protocol {
        if self.0.len() == 0 {
            return UNKNOWN;
        }
        self.0[0]
    }

    pub fn payload(&self) -> &[u8] {
        &self.0[1..]
    }
}
