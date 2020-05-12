// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// Num of bits to store small block value
pub const SMALL_BLOCK_LENGTH: usize = 0x04;

/// Bitwise shift size for pack block value
pub const PACK_BYTE_SHIFT: usize = 0x07;

/// Num of bits in the byte
pub const BYTE_BITS_COUNT: usize = 0x08;

/// Value of the long block
pub const LONG_BLOCK_VALUE: usize = 0x10;

/// Bitwise mask for unpack long block value
pub const UNPACK_BYTE_MASK: usize = 0x7F;

/// MSB value
pub const BYTE_SLICE_VALUE: usize = 0x80;

/// Maximum object size to encode or decode
pub const OBJECT_MAX_SIZE: usize = 0x100_000;
