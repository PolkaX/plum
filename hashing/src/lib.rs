// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The common hash functions.

#![deny(missing_docs)]

use blake2b_simd::Params as Blake2b;
use blake2s_simd::Params as Blake2s;
use digest::Digest;

/// Generates blake2b hash with provided size.
///
/// # Example
/// ```
/// use plum_hashing::blake2b_variable;
///
/// let data: Vec<u8> = vec![];
/// let hash = blake2b_variable(&data, 20);
/// assert_eq!(hash.len(), 20);
/// ```
pub fn blake2b_variable<T: AsRef<[u8]>>(data: T, length: usize) -> Vec<u8> {
    assert!(length <= blake2b_simd::OUTBYTES);
    let hash = Blake2b::new()
        .hash_length(length)
        .to_state()
        .update(data.as_ref())
        .finalize();

    let res = hash.as_bytes().to_vec();
    assert_eq!(res.len(), length);
    res
}

/// Generates blake2b hash of fixed 32 bytes size.
///
/// # Example
/// ```
/// use plum_hashing::blake2b_256;
///
/// let data: Vec<u8> = vec![];
/// let hash = blake2b_256(&data);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn blake2b_256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let hash = Blake2b::new()
        .hash_length(32)
        .to_state()
        .update(data.as_ref())
        .finalize();

    let mut res = [0u8; 32];
    res.copy_from_slice(hash.as_bytes());
    res
}

/// Generates blake2b hash of fixed 64 bytes size.
///
/// # Example
/// ```
/// use plum_hashing::blake2b_512;
///
/// let data: Vec<u8> = vec![];
/// let hash = blake2b_512(&data);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn blake2b_512<T: AsRef<[u8]>>(data: T) -> [u8; 64] {
    let hash = Blake2b::new()
        .hash_length(64)
        .to_state()
        .update(data.as_ref())
        .finalize();

    *hash.as_array()
}

/// Generates blake2b hash with provided size.
///
/// # Example
/// ```
/// use plum_hashing::blake2s_variable;
///
/// let data: Vec<u8> = vec![];
/// let hash = blake2s_variable(&data, 20);
/// assert_eq!(hash.len(), 20);
/// ```
pub fn blake2s_variable<T: AsRef<[u8]>>(data: T, length: usize) -> Vec<u8> {
    assert!(length <= blake2s_simd::OUTBYTES);
    let hash = Blake2s::new()
        .hash_length(length)
        .to_state()
        .update(data.as_ref())
        .finalize();

    let res = hash.as_bytes().to_vec();
    assert_eq!(res.len(), length);
    res
}

/// Generates blake2s hash of fixed 16 bytes size.
///
/// # Example
/// ```
/// use plum_hashing::blake2s_128;
///
/// let data: Vec<u8> = vec![];
/// let hash = blake2s_128(&data);
/// assert_eq!(hash.len(), 16);
/// ```
pub fn blake2s_128<T: AsRef<[u8]>>(data: T) -> [u8; 16] {
    let hash = Blake2s::new()
        .hash_length(16)
        .to_state()
        .update(data.as_ref())
        .finalize();

    let mut res = [0u8; 16];
    res.copy_from_slice(hash.as_bytes());
    res
}

/// Generates blake2s hash of fixed 32 bytes size.
///
/// # Example
/// ```
/// use plum_hashing::blake2s_256;
///
/// let data: Vec<u8> = vec![];
/// let hash = blake2s_256(&data);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn blake2s_256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let hash = Blake2s::new()
        .hash_length(32)
        .to_state()
        .update(data.as_ref())
        .finalize();

    *hash.as_array()
}

/// Generates sha1 hash.
///
/// **Note**: Please DO NOT use sha1 hash function.
///
/// As of 2020, attacks against SHA-1 are as practical as against MD5; as such, it is recommended
/// to remove SHA-1 from products as soon as possible and use instead SHA-256 or SHA-3.
/// Replacing SHA-1 is urgent where it is used for signatures.
///
/// # Example
/// ```
/// use plum_hashing::sha1;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha1(&data);
/// assert_eq!(hash.len(), 20);
/// ```
pub fn sha1<T: AsRef<[u8]>>(data: T) -> [u8; 20] {
    let hash = sha1::Sha1::digest(data.as_ref());
    let mut res = [0u8; 20];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha224 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha224;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha224(&data);
/// assert_eq!(hash.len(), 28);
/// ```
pub fn sha224<T: AsRef<[u8]>>(data: T) -> [u8; 28] {
    let hash = sha2::Sha224::digest(data.as_ref());
    let mut res = [0u8; 28];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha256 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha256;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha256(&data);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn sha256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let hash = sha2::Sha256::digest(data.as_ref());
    let mut res = [0u8; 32];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha384 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha384;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha384(&data);
/// assert_eq!(hash.len(), 48);
/// ```
pub fn sha384<T: AsRef<[u8]>>(data: T) -> [u8; 48] {
    let hash = sha2::Sha384::digest(data.as_ref());
    let mut res = [0u8; 48];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha512 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha512;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha512(&data);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn sha512<T: AsRef<[u8]>>(data: T) -> [u8; 64] {
    let hash = sha2::Sha512::digest(data.as_ref());
    let mut res = [0u8; 64];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha3_224 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha3_224;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha3_224(&data);
/// assert_eq!(hash.len(), 28);
/// ```
pub fn sha3_224<T: AsRef<[u8]>>(data: T) -> [u8; 28] {
    let hash = sha3::Sha3_224::digest(data.as_ref());
    let mut res = [0u8; 28];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha3_256 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha3_256;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha3_256(&data);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn sha3_256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let hash = sha3::Sha3_256::digest(data.as_ref());
    let mut res = [0u8; 32];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha3_384 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha3_384;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha3_384(&data);
/// assert_eq!(hash.len(), 48);
/// ```
pub fn sha3_384<T: AsRef<[u8]>>(data: T) -> [u8; 48] {
    let hash = sha3::Sha3_384::digest(data.as_ref());
    let mut res = [0u8; 48];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates sha3_512 hash.
///
/// # Example
/// ```
/// use plum_hashing::sha3_512;
///
/// let data: Vec<u8> = vec![];
/// let hash = sha3_512(&data);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn sha3_512<T: AsRef<[u8]>>(data: T) -> [u8; 64] {
    let hash = sha3::Sha3_512::digest(data.as_ref());
    let mut res = [0u8; 64];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates keccak224 hash.
///
/// # Example
/// ```
/// use plum_hashing::keccak224;
///
/// let data: Vec<u8> = vec![];
/// let hash = keccak224(&data);
/// assert_eq!(hash.len(), 28);
/// ```
pub fn keccak224<T: AsRef<[u8]>>(data: T) -> [u8; 28] {
    let hash = sha3::Keccak224::digest(data.as_ref());
    let mut res = [0u8; 28];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates keccak256 hash.
///
/// # Example
/// ```
/// use plum_hashing::keccak256;
///
/// let data: Vec<u8> = vec![];
/// let hash = keccak256(&data);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn keccak256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let hash = sha3::Keccak256::digest(data.as_ref());
    let mut res = [0u8; 32];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates keccak384 hash.
///
/// # Example
/// ```
/// use plum_hashing::keccak384;
///
/// let data: Vec<u8> = vec![];
/// let hash = keccak384(&data);
/// assert_eq!(hash.len(), 48);
/// ```
pub fn keccak384<T: AsRef<[u8]>>(data: T) -> [u8; 48] {
    let hash = sha3::Keccak384::digest(data.as_ref());
    let mut res = [0u8; 48];
    res.copy_from_slice(hash.as_slice());
    res
}

/// Generates keccak512 hash.
///
/// # Example
/// ```
/// use plum_hashing::keccak512;
///
/// let data: Vec<u8> = vec![];
/// let hash = keccak512(&data);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn keccak512<T: AsRef<[u8]>>(data: T) -> [u8; 64] {
    let hash = sha3::Keccak512::digest(data.as_ref());
    let mut res = [0u8; 64];
    res.copy_from_slice(hash.as_slice());
    res
}
