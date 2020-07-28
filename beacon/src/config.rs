// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use bls::{PublicKey, Serialize};

use plum_hash::H256;

/// The network type of Drand network.
#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
pub enum DrandNetwork {
    Mainnet,
    Testnet,
    Devnet,
}

impl Default for DrandNetwork {
    fn default() -> Self {
        Self::Testnet
    }
}

impl DrandNetwork {
    /// Return the Drand config according to the network type.
    pub fn config(self) -> DrandConfig {
        match self {
            DrandNetwork::Mainnet => DrandConfig::mainnet(),
            DrandNetwork::Testnet => DrandConfig::testnet(),
            DrandNetwork::Devnet => DrandConfig::devnet(),
        }
    }
}

/// The configuration of the Drand network.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct DrandConfig {
    pub servers: Vec<&'static str>,
    pub relays: Vec<&'static str>,
    pub chain_info: DrandChainInfo,
}

/// The information of the Drand chain.
#[derive(Clone, Debug)]
pub struct DrandChainInfo {
    pub(crate) public_key: PublicKey,
    pub(crate) period: u64,
    pub(crate) genesis_time: u64,

    pub(crate) hash: H256,
    pub(crate) group_hash: H256,
}

impl DrandConfig {
    /// Return the config of the Drand main network.
    pub fn mainnet() -> Self {
        Self {
            servers: vec![
                "https://api.drand.sh",
                "https://api2.drand.sh",
                "https://api3.drand.sh",
            ],
            relays: vec![
                "/dnsaddr/api.drand.sh/",
                "/dnsaddr/api2.drand.sh/",
                "/dnsaddr/api3.drand.sh/",
            ],
            chain_info: DrandChainInfo {
                public_key: pubkey("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31"),
                period: 30,
                genesis_time: 1595431050,
                hash: "8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce".parse().unwrap(),
                group_hash: "176f93498eac9ca337150b46d21dd58673ea4e3581185f869672e59fa4cb390a".parse().unwrap(),
            }
        }
    }

    /// Return the config of the Drand test network.
    pub fn testnet() -> Self {
        Self {
            servers: vec![
                "https://pl-eu.testnet.drand.sh",
                "https://pl-us.testnet.drand.sh",
                "https://pl-sin.testnet.drand.sh",
            ],
            relays: vec![
                "/dnsaddr/pl-eu.testnet.drand.sh/",
                "/dnsaddr/pl-us.testnet.drand.sh/",
                "/dnsaddr/pl-sin.testnet.drand.sh/",
            ],
            chain_info: DrandChainInfo {
                public_key: pubkey("922a2e93828ff83345bae533f5172669a26c02dc76d6bf59c80892e12ab1455c229211886f35bb56af6d5bea981024df"),
                period: 25,
                genesis_time: 1590445175,
                hash: "84b2234fb34e835dccd048255d7ad3194b81af7d978c3bf157e3469592ae4e02".parse().unwrap(),
                group_hash: "4dd408e5fdff9323c76a9b6f087ba8fdc5a6da907bd9217d9d10f2287d081957".parse().unwrap(),
            }
        }
    }

    /// Return the config of the Drand develop network.
    pub fn devnet() -> Self {
        Self {
            servers: vec![
                "https://dev1.drand.sh",
                "https://dev2.drand.sh",
            ],
            relays: vec![
                "/dnsaddr/dev1.drand.sh/",
                "/dnsaddr/dev2.drand.sh/",
            ],
            chain_info: DrandChainInfo {
                public_key: pubkey("8cda589f88914aa728fd183f383980b35789ce81b274e5daee1f338b77d02566ef4d3fb0098af1f844f10f9c803c1827"),
                period: 25,
                genesis_time: 1595348225,
                hash: "e73b7dc3c4f6a236378220c0dd6aa110eb16eed26c11259606e07ee122838d4f".parse().unwrap(),
                group_hash: "567d4785122a5a3e75a9bc9911d7ea807dd85ff76b78dc4ff06b075712898607".parse().unwrap(),
            }
        }
    }
}

fn pubkey(s: &str) -> PublicKey {
    let raw = hex::decode(s).unwrap();
    PublicKey::from_bytes(&raw).unwrap()
}
