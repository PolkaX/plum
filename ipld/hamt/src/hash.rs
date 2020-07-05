// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

#[cfg(not(feature = "identity-hash"))]
pub(crate) fn hash<T: AsRef<[u8]>>(bytes: T) -> [u8; 8] {
    let hash = murmur3::murmur3_x64_128(&mut bytes.as_ref(), 0).unwrap();
    let hash = hash.to_be_bytes();
    let mut result = [0u8; 8];
    result.copy_from_slice(&hash[8..]);
    result
}

#[cfg(feature = "identity-hash")]
pub(crate) fn hash<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut hash = [0u8; 32];
    for (index, byte) in bytes.as_ref().iter().take(32).enumerate() {
        hash[index] = *byte;
    }
    hash
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "identity-hash"))]
    fn test_hash() {
        let hash = hash(b"abcd");
        assert_eq!(hash, [184, 123, 183, 214, 70, 86, 205, 79]);
    }

    #[test]
    #[cfg(feature = "identity-hash")]
    fn test_hash() {
        let hash = hash(b"abcd");
        assert_eq!(
            hash,
            [
                b'a', b'b', b'c', b'd', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
