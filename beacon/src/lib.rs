// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of the Drand Beacon client for providing randomness.

#![deny(missing_docs)]

extern crate bls_signatures as bls;

mod config;
mod drand;
mod mock;

pub use self::config::{DrandConfig, DrandNetwork};
pub use self::drand::{DrandBeacon, RandomBeacon};
pub use self::mock::MockBeacon;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_drand_beacon_mainnet() {
        let beacon = DrandBeacon::new(100, 25, DrandConfig::mainnet()).unwrap();
        let entry = beacon.entry(0).await.unwrap();
        println!("Mainnet round 0: {:?}", entry);
        let entry = beacon.entry(1).await.unwrap();
        println!("Mainnet round 1: {:?}", entry);
    }

    #[tokio::test]
    async fn test_drand_beacon_testnet() {
        let beacon = DrandBeacon::new(100, 25, DrandConfig::testnet()).unwrap();
        let entry = beacon.entry(0).await.unwrap();
        println!("Testnet round 0: {:?}", entry);
        let entry = beacon.entry(1).await.unwrap();
        println!("Testnet round 1: {:?}", entry);
    }
}
