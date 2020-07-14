// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use anyhow::Result;
use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipld::{IpldStore, IpldValue};

use crate::{max_leaf_node_size_for, max_leaf_value_size_for};

///
#[derive(Clone, PartialEq, Debug)]
pub enum Link {
    ///
    Cid(Cid),
    ///
    Cache(Box<Node>),
}

/// Each node in an IPLD vector stores the width, the height of the node,
/// starting from 0 where values are stored,
/// and a data array to contain values (for height 0), or child node CIDs (for heights above 1).
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    ///
    Links(Vec<Link>),
    ///
    Leaves(Vec<IpldValue>),
}

impl Default for Node {
    fn default() -> Self {
        Node::Leaves(vec![])
    }
}

// Implement CBOR serialization for Node.
impl encode::Encode for Node {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        todo!()
    }
}

// Implement CBOR deserialization for Node.
impl<'b> decode::Decode<'b> for Node {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        todo!()
    }
}

impl Node {
    ///
    pub fn get<S: IpldStore>(
        &self,
        store: &S,
        height: u64,
        index: usize,
    ) -> Result<Option<&IpldValue>> {
        let sub_index = index / max_leaf_node_size_for(height);

        match self {
            Node::Leaves(values) => Ok(values.get(index)),
            Node::Links(links) => match &links[sub_index] {
                Link::Cid(cid) => todo!(),
                Link::Cache(node) => {
                    node.get(store, height - 1, index % max_leaf_node_size_for(height))
                }
            },
        }
    }

    ///
    pub fn set<S: IpldStore>(
        &mut self,
        store: &S,
        height: u64,
        index: usize,
        value: IpldValue,
    ) -> Result<bool> {
        todo!()
    }

    pub fn delete<S: IpldStore>(
        &mut self,
        store: &S,
        height: u64,
        index: usize,
    ) -> Result<Option<IpldValue>> {
        todo!()
    }

    ///
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    ///
    pub fn flush<S: IpldStore>(&mut self, store: &mut S, height: u64) -> Result<()> {
        if let Node::Links(links) = self {
            for link in links.iter_mut() {
                if let Link::Cache(node) = link {
                    node.flush(store, height - 1)?;
                    let cid = IpldStore::put(store, node)?;
                    *link = Link::Cid(cid);
                }
            }
        }

        Ok(())
    }
}
