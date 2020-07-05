// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};

use ipfs_blockstore::BlockStore;
use ipld::IpldValue;

use crate::bitfield::U256;
use crate::error::HamtError;
use crate::pointer::Pointer;

///
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Node {
    bitfield: U256,
    pointers: Vec<Pointer>,
}

// Implement CBOR serialization for Node.
impl encode::Encode for Node {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(2)?
            .encode(&self.bitfield)?
            .encode(&self.pointers)?
            .ok()
    }
}

// Implement CBOR deserialization for Node.
impl<'b> decode::Decode<'b> for Node {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(2));
        Ok(Node {
            bitfield: d.decode()?,
            pointers: d.decode()?,
        })
    }
}

impl Node {
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    pub fn set<'a, BS>(
        &mut self,
        key: &str,
        value: IpldValue,
        store: &'a BS,
        bit_width: usize,
    ) -> Result<(), HamtError>
    where
        BS: BlockStore,
    {
        todo!()
    }

    ///
    pub fn get<'a, BS>(
        &self,
        key: &str,
        store: &'a BS,
        bit_width: usize,
    ) -> Result<Option<IpldValue>, HamtError>
    where
        BS: BlockStore,
    {
        todo!()
    }

    ///
    pub fn remove<'a, BS>(
        &mut self,
        key: &str,
        store: &'a BS,
        bit_width: usize,
    ) -> Result<Option<(String, IpldValue)>, HamtError>
    where
        BS: BlockStore,
    {
        todo!()
    }

    ///
    pub fn contains(&self, key: &str) -> Result<bool, HamtError> {
        todo!()
    }

    ///
    pub fn flush<BS: BlockStore>(&mut self, store: &BS) -> Result<(), HamtError> {
        todo!()
    }

    ///
    pub fn for_each<BS, F, V>(&self, store: &BS, f: &mut F) -> Result<(), HamtError>
    where
        BS: BlockStore,
        F: FnMut(&str, V) -> Result<(), HamtError>,
        V: for<'b> minicbor::Decode<'b>,
    {
        todo!()
    }

    fn insert_child(&mut self, idx: usize, key: &[u8], value: IpldValue) {
        todo!()
    }

    fn remove_child(&mut self, idx: usize) -> Pointer {
        todo!()
    }

    fn get_child(&self, idx: usize) -> &Pointer {
        &self.pointers[idx]
    }

    fn get_child_mut(&mut self, idx: usize) -> &mut Pointer {
        &mut self.pointers[idx]
    }
}
