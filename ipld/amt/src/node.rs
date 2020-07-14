// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipld::{IpldStore, IpldValue};

use crate::bitmap::BitMap;
use crate::error::{IpldAmtError, Result};
use crate::{max_leaf_node_size_for, max_leaf_value_size_for, WIDTH};

///
#[derive(Clone, PartialEq, Debug)]
pub enum Link {
    ///
    Cache(Box<Node>),
    ///
    Cid(Cid),
}

impl Link {
    fn load_node<S: IpldStore>(&mut self, store: &S) -> Result<()> {
        if let Link::Cid(cid) = self {
            let node =
                IpldStore::get::<Node>(store, cid)?.ok_or_else(|| IpldAmtError::CidNotFound)?;
            *self = Link::Cache(Box::new(node));
        }
        Ok(())
    }
}

/// Each node in an IPLD vector stores the width, the height of the node,
/// starting from 0 where values are stored,
/// and a data array to contain values (for height 0), or child node CIDs (for heights above 1).
#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    ///
    Links {
        bitmap: BitMap,
        links: [Option<Link>; WIDTH],
    },
    ///
    Leaves {
        bitmap: BitMap,
        values: [Option<IpldValue>; WIDTH],
    },
}

/*
enum NodeItem {
    Links(Vec<Link>),
    Leaves(Vec<IpldValue>),
}
*/

impl Default for Node {
    fn default() -> Self {
        Node::Leaves {
            bitmap: BitMap::default(),
            values: [None; WIDTH],
        }
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
    pub fn bitmap(&self) -> BitMap {
        match self {
            Node::Links { bitmap, .. } => *bitmap,
            Node::Leaves { bitmap, .. } => *bitmap,
        }
    }

    ///
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    ///
    pub fn get<S: IpldStore>(
        &self,
        store: &S,
        height: u64,
        index: usize,
    ) -> Result<Option<IpldValue>> {
        // the sub index at height - 1.
        let sub_index = index / max_leaf_node_size_for(height);
        assert!(sub_index <= WIDTH);

        if !self.bitmap().has_bit(sub_index as u8) {
            return Ok(None);
        }

        match self {
            Node::Leaves { values, .. } => Ok(values[index].clone()),
            Node::Links { links, .. } => match &links[sub_index] {
                Some(Link::Cid(cid)) => {
                    let node = IpldStore::get::<Node>(store, cid)?
                        .ok_or_else(|| IpldAmtError::CidNotFound)?;
                    node.get(store, height - 1, index % max_leaf_node_size_for(height))
                }
                Some(Link::Cache(node)) => {
                    node.get(store, height - 1, index % max_leaf_node_size_for(height))
                }
                None => Ok(None),
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
        if height == 0 {}

        // the sub index at height - 1.
        let sub_index = index / max_leaf_node_size_for(height);
        assert!(sub_index <= WIDTH);

        match self {
            Node::Links { bitmap, links } => {
                links[sub_index] = match &links[sub_index] {
                    Some(Link::Cache(node)) => Some(Link::Cache(node.clone())),
                    Some(Link::Cid(cid)) => {
                        let mut node = IpldStore::get::<Node>(store, cid)?
                            .ok_or_else(|| IpldAmtError::CidNotFound)?;
                        Some(Link::Cache(Box::new(node)));
                    }
                    None => {
                        let mut node = match height {
                            1 => Node::Leaves {
                                bitmap: BitMap::default(),
                                values: [None; WIDTH],
                            },
                            _ => Node::Links {
                                bitmap: BitMap::default(),
                                links: [None; WIDTH],
                            },
                        };
                        bitmap.set_bit(sub_index as u8);
                        Some(Link::Cache(Box::new(node)));
                    }
                };
            }
            Node::Leaves { .. } => {
                unreachable!("If the current node's height > 0, it sohuldn't be leaf")
            }
        }
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
    pub fn flush<S: IpldStore>(&mut self, store: &mut S, height: u64) -> Result<()> {
        todo!()
    }
}
