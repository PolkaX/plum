// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_address::{Address, Protocol};
use plum_hashing::sha256;

use crate::errors::CryptoError;

/// The `BLS` public key for verifying VRF.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VrfPublicKey(bls::PublicKey);

impl VrfPublicKey {
    /// Unwrap the VRF public key to the `BLS` public key.
    pub fn into_inner(self) -> bls::PublicKey {
        self.0
    }

    /// Create the VRF public key from the raw bytes.
    pub fn from_bytes<T: AsRef<[u8]>>(raw: T) -> Result<Self, CryptoError> {
        use bls::Serialize;
        Ok(VrfPublicKey(bls::PublicKey::from_bytes(raw.as_ref())?))
    }

    /// Return a byte slice of this `VRF public key`'s contents.
    pub fn as_bytes(&self) -> Vec<u8> {
        use bls::Serialize;
        self.0.as_bytes()
    }
}

impl From<bls::PublicKey> for VrfPublicKey {
    fn from(bls_pubkey: bls::PublicKey) -> Self {
        VrfPublicKey(bls_pubkey)
    }
}

/// The `BLS` private key for computing VRF.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VrfPrivateKey(bls::PrivateKey);

impl VrfPrivateKey {
    /// Unwrap the VRF private key to the `BLS` private key.
    pub fn into_inner(self) -> bls::PrivateKey {
        self.0
    }

    /// Create the VRF private key from the raw bytes.
    pub fn from_bytes<T: AsRef<[u8]>>(raw: T) -> Result<Self, CryptoError> {
        use bls::Serialize;
        Ok(VrfPrivateKey(bls::PrivateKey::from_bytes(raw.as_ref())?))
    }

    /// Return a byte slice of this `VRF private key`'s contents.
    pub fn as_bytes(&self) -> Vec<u8> {
        use bls::Serialize;
        self.0.as_bytes()
    }
}

impl From<bls::PrivateKey> for VrfPrivateKey {
    fn from(bls_privkey: bls::PrivateKey) -> Self {
        VrfPrivateKey(bls_privkey)
    }
}

/// The `BLS` signature.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VrfProof(bls::Signature);

impl VrfProof {
    /// Unwrap the VRF proof key to the `BLS` signature.
    pub fn into_inner(self) -> bls::Signature {
        self.0
    }

    /// Create the VRF proof from the raw bytes.
    pub fn from_bytes<T: AsRef<[u8]>>(raw: T) -> Result<Self, CryptoError> {
        use bls::Serialize;
        Ok(VrfProof(bls::Signature::from_bytes(raw.as_ref())?))
    }

    /// Return a byte slice of this `VRF proof`'s contents.
    pub fn as_bytes(&self) -> Vec<u8> {
        use bls::Serialize;
        self.0.as_bytes()
    }
}

impl From<bls::Signature> for VrfProof {
    fn from(bls_signature: bls::Signature) -> Self {
        VrfProof(bls_signature)
    }
}

/// Computing VRF with the given `BLS` private key and VRF params(miner address must be ID address).
///
/// Return the `BLS` signature.
pub fn compute_vrf<M>(
    privkey: &VrfPrivateKey,
    personalization: u64,
    msg: M,
    miner: &Address,
) -> VrfProof
where
    M: AsRef<[u8]>,
{
    let msg = hash_vrf_base(personalization, msg, miner);
    let signature = privkey.0.sign(msg);
    VrfProof(signature)
}

/// Verify VRF with the given `BLS` public key, VRF params(miner address must be ID address)
/// and `BLS` signature.
///
/// Return the result of VRF verification.
pub fn verify_vrf<M>(
    pubkey: &VrfPublicKey,
    personalization: u64,
    msg: M,
    miner: &Address,
    proof: &VrfProof,
) -> bool
where
    M: AsRef<[u8]>,
{
    let msg = hash_vrf_base(personalization, msg, miner);
    // When signing with `BLS` privkey, the message will be hashed in `bls::PrivateKey::sign`,
    // so the message here needs to be hashed before the signature is verified.
    let hashed_msg = bls::hash(msg.as_ref());
    bls::verify(&proof.0, &[hashed_msg], &[pubkey.0])
}

fn hash_vrf_base<M>(personalization: u64, msg: M, miner: &Address) -> [u8; 32]
where
    M: AsRef<[u8]>,
{
    assert_eq!(
        miner.protocol(),
        Protocol::Id,
        "Miner address must be a ID address"
    );
    let miner_bytes = miner.as_bytes();
    let mut bytes = Vec::<u8>::with_capacity(8 + 1 + msg.as_ref().len() + 1 + miner_bytes.len());
    bytes.extend(&personalization.to_le_bytes());
    bytes.push(0u8);
    bytes.extend(msg.as_ref());
    bytes.push(0u8);
    bytes.extend(miner_bytes);
    sha256(bytes)
}
