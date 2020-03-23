// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
};

use num::traits::NumAssign;

///
pub trait Number:
    NumAssign
    + Shl<Output = Self>
    + ShlAssign
    + Shr<Output = Self>
    + ShrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Clone
    + Copy
    + Cast<usize>
    + Cast<u8>
{
}

impl<T> Number for T where
    T: NumAssign
        + Shl<Output = Self>
        + ShlAssign
        + Shr<Output = Self>
        + ShrAssign
        + BitAnd<Output = Self>
        + BitAndAssign
        + BitOr<Output = Self>
        + BitOrAssign
        + BitXor<Output = Self>
        + BitXorAssign
        + Eq
        + PartialEq
        + Ord
        + PartialOrd
        + Clone
        + Copy
        + Cast<usize>
        + Cast<u8>
{
}

///
pub trait Cast<T>: Sized {
    fn into(self) -> T;
    fn from(_: T) -> Self;
}

macro_rules! cast_impl {
    ($($src:ty => $($dst: ty),+);+;) => {
        $(
            $(
                impl Cast<$src> for $dst {
                    #[inline]
                    fn into(self) -> $src {
                        self as $src
                    }
                    #[inline]
                    fn from(s: $src) -> $dst {
                        s as $dst
                    }
                }
            )+
        )+
    }
}

cast_impl! {
    u8    => u8, u16, u32, usize, u64;
    u16   => u8, u16, u32, usize, u64;
    u32   => u8, u16, u32, usize, u64;
    usize => u8, u16, u32, usize, u64;
    u64   => u8, u16, u32, usize, u64;
}
