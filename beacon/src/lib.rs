// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of the Drand Beacon client for providing randomness.

#![deny(missing_docs)]

extern crate bls_signatures as bls;

mod beacon;
mod config;
mod mock;
#[cfg(feature = "grpc")]
mod proto;

pub use self::beacon::{DrandBeacon, RandomBeacon};
pub use self::config::{DrandConfig, DrandNetwork};
pub use self::mock::MockBeacon;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_drand_beacon() {
        let beacon = DrandBeacon::new(100, 25, DrandConfig::mainnet()).unwrap();
        let entry = beacon.entry(1).await.unwrap();
        println!("{:?}", entry);
    }
}
