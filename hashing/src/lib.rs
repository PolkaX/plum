// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use blake2_rfc::blake2b::Blake2b;

/// Generates blake2b hash with provided size.
///
/// # Example
/// ```
/// use plum_hashing::blake2b_variable;
///
/// let ingest: Vec<u8> = vec![];
/// let hash = blake2b_variable(&ingest, 20);
/// assert_eq!(hash.len(), 20);
/// ```
pub fn blake2b_variable<I: AsRef<[u8]>>(ingest: I, size: usize) -> Vec<u8> {
    let mut context = Blake2b::new(size);
    context.update(ingest.as_ref());
    let hash = context.finalize();
    hash.as_bytes().to_vec()
}

/// Generates blake2b hash of fixed 32 bytes size.
///
/// # Example
/// ```
/// use plum_hashing::blake2b_256;
///
/// let ingest: Vec<u8> = vec![];
/// let hash = blake2b_256(&ingest);
/// assert_eq!(hash.len(), 32);
/// ```
pub fn blake2b_256<I: AsRef<[u8]>>(ingest: I) -> [u8; 32] {
    let mut context = Blake2b::new(32);
    context.update(ingest.as_ref());
    let hash = context.finalize();

    let mut res = [0u8; 32];
    res.copy_from_slice(hash.as_bytes());
    res
}
