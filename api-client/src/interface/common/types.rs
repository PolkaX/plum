// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::fmt;

use libp2p_core::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The permission of API.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    /// Read-only permission
    Read,
    /// Write permission
    Write,
    /// Use wallet keys for signing
    Sign,
    /// Manage permissions
    Admin,
}

impl Default for Permission {
    fn default() -> Self {
        Permission::Read
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Permission::Read => f.write_str("read"),
            Permission::Write => f.write_str("write"),
            Permission::Sign => f.write_str("sign"),
            Permission::Admin => f.write_str("admin"),
        }
    }
}

/// Connectedness signals the capacity for a connection with a given node.
/// It is used to signal to services and other peers whether a node is reachable.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum Connectedness {
    /// NotConnected means no connection to peer, and no extra information (default)
    NotConnected = 0,
    /// Connected means has an open, live connection to peer
    Connected = 1,
    /// CanConnect means recently connected to peer, terminated gracefully
    CanConnect = 2,
    /// CannotConnect means recently attempted connecting but failed to connect.
    /// (should signal "made effort, failed")
    CannotConnect = 3,
}

/// AddrInfo is a small struct used to pass around a peer with a set of addresses.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PeerAddrInfo {
    /// peer ID.
    #[serde(rename = "ID")]
    #[serde(with = "crate::helper::peer_id")]
    pub id: PeerId,
    /// A set of addresses.
    pub addrs: Vec<Multiaddr>,
}

/// Version provides various build-time information.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    /// User version (build version + current commit)
    pub version: String,
    /// api_version is a semver version of the rpc api exposed
    #[serde(rename = "APIVersion")]
    pub api_version: BuildVersion,
    /// Seconds
    pub block_delay: u64,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+api{}", self.version, self.api_version)
    }
}

/// BuildVersion is the local build version, set by build system
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuildVersion(u32);

impl fmt::Display for BuildVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (major, minor, patch) = self.semver();
        write!(f, "{}.{}.{}", major, minor, patch)
    }
}

impl From<(u8, u8, u8)> for BuildVersion {
    fn from((major, minor, patch): (u8, u8, u8)) -> Self {
        Self::new((major, minor, patch))
    }
}

impl BuildVersion {
    /// Create a new build version.
    pub fn new((major, minor, patch): (u8, u8, u8)) -> Self {
        Self(u32::from(major) << 16 | u32::from(minor) << 8 | u32::from(patch))
    }

    /// Return the version with the (major, minor, patch) format.
    pub fn semver(self) -> (u8, u8, u8) {
        (self.major(), self.minor(), self.patch())
    }

    /// Return the major version.
    pub fn major(self) -> u8 {
        (self.0 & MAJOR_ONLY_MASK >> 16) as u8
    }

    /// Return the minor version.
    pub fn minor(self) -> u8 {
        ((self.0 & MINOR_ONLY_MASK) >> 8) as u8
    }

    /// Return the patch version.
    pub fn patch(self) -> u8 {
        (self.0 & PATCH_ONLY_MASK) as u8
    }
}

const MAJOR_ONLY_MASK: u32 = 0x00ff_0000;
const MINOR_ONLY_MASK: u32 = 0x0000_ff00;
const PATCH_ONLY_MASK: u32 = 0x0000_00ff;
