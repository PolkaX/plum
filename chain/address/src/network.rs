// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// The default network type.
pub const NETWORK_DEFAULT: Network = Network::Test;

pub(crate) const NETWORK_MAINNET_PREFIX: &str = "f";
pub(crate) const NETWORK_TESTNET_PREFIX: &str = "t";

/// The network type used by the address.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Network {
    /// Main network, prefix: 'f'.
    Main,
    /// Test network, prefix: 't'.
    Test,
}

impl Default for Network {
    fn default() -> Self {
        Network::Test
    }
}

impl Network {
    /// Return the prefix identifier of network.
    pub fn prefix(self) -> &'static str {
        match self {
            Network::Main => NETWORK_MAINNET_PREFIX,
            Network::Test => NETWORK_TESTNET_PREFIX,
        }
    }
}
