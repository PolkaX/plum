// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};

use ipld::{IpldStore, IpldValue};

use crate::bitmap::BitMap;
use crate::error::{IpldAmtError, Result};
use crate::{max_leaf_node_size_for, max_leaf_value_size_for, WIDTH};

/// The link to Amt node.
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Link {
    Cache(Box<Node>),
    Cid(Cid),
}

impl Link {
    pub fn load_node<S: IpldStore>(&self, store: &S) -> Result<Node> {
        Ok(match self {
            Link::Cache(node) => *node.clone(),
            Link::Cid(cid) => {
                IpldStore::get::<Node>(store, cid)?.ok_or_else(|| IpldAmtError::CidNotFound)?
            }
        })
    }
}

/// Each node in an IPLD vector stores the width, the height of the node,
/// starting from 0 where values are stored,
/// and a data array to contain values (for height 0), or child node CIDs (for heights above 1).
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Node {
    Link {
        bitmap: BitMap,
        links: [Option<Link>; WIDTH],
    },
    Leaf {
        bitmap: BitMap,
        values: [Option<IpldValue>; WIDTH],
    },
}

impl Default for Node {
    fn default() -> Self {
        Node::Leaf {
            bitmap: BitMap::default(),
            values: <[Option<IpldValue>; WIDTH]>::default(),
        }
    }
}

fn links_as_cids(links: &[Option<Link>; WIDTH]) -> Vec<&Cid> {
    let cids = links
        .iter()
        .filter_map(|link| match link {
            Some(Link::Cid(cid)) => Some(cid),
            Some(Link::Cache(_)) => None,
            None => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(cids.len(), WIDTH);
    cids
}

// Implement CBOR serialization for Node.
impl encode::Encode for Node {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        match self {
            Node::Link { bitmap, links } => e
                .encode(bitmap)?
                .encode(links_as_cids(links))?
                .encode([0u8; WIDTH])?
                .ok(),
            Node::Leaf { bitmap, values } => e.encode(bitmap)?.encode([0u8; WIDTH])?.encode()?.ok(),
        }
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
            Node::Link { bitmap, .. } => *bitmap,
            Node::Leaf { bitmap, .. } => *bitmap,
        }
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.bitmap().is_empty()
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
            Node::Leaf { values, .. } => Ok(values[index].clone()),
            Node::Link { links, .. } => match &links[sub_index] {
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
        todo!()
        /*
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
                            1 => Node::Leaf {
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
            Node::Leaf { .. } => {
                unreachable!("If the current node's height > 0, it sohuldn't be leaf")
            }
        }*/
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
