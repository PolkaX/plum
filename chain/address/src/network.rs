// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// The default network type.
pub const NETWORK_DEFAULT: Network = Network::Test;

pub(crate) const NETWORK_MAINNET_PREFIX: &str = "f";
pub(crate) const NETWORK_TESTNET_PREFIX: &str = "t";

/// # Safety
/// this function should set at the beginning of programing, and only set once.
/// could not change it in runtime.
pub unsafe fn set_network(network: Network) {
    if network != NETWORK_DEFAULT {
        let n = &mut *(&NETWORK_DEFAULT as *const Network as *mut Network);
        *n = network;
    }
}

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
