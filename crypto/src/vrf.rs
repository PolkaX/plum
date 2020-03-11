// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_address::{Address, Protocol};
use plum_hashing::sha256;

/// The bls public key for verifying VRF.
pub type VrfPublicKey = bls::PublicKey;

/// The bls private key for computing VRF.
pub type VrfPrivateKey = bls::PrivateKey;

/// The bls signature.
pub type VrfProof = bls::Signature;

/// Computing VRF with the given bls private key and VRF params(miner address must be ID address).
///
/// Return the bls signature.
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
    let signature = privkey.sign(msg);
    signature
}

/// Verify VRF with the given bls public key, VRF params(miner address must be ID address)
/// and bls signature.
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
    // When signing with bls privkey, the message will be hashed in `bls::PrivateKey::sign`,
    // so the message here needs to be hashed before the signature is verified.
    let hashed_msg = bls::hash(msg.as_ref());
    bls::verify(&proof, &[hashed_msg], &[*pubkey])
}

fn hash_vrf_base<M>(personalization: u64, msg: M, miner: &Address) -> [u8; 32]
where
    M: AsRef<[u8]>,
{
    assert_eq!(
        miner.protocol(),
        Protocol::ID,
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
