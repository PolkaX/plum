// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use plum_bigint::BigInt;
use plum_block::{BlockHeader, Ticket};

use crate::errors::TipsetError;
use crate::key::TipsetKey;

///
#[derive(Clone, Debug)]
pub struct Tipset {
    key: TipsetKey,
    blocks: Vec<BlockHeader>,
    height: u64,
}

impl PartialEq for Tipset {
    fn eq(&self, other: &Self) -> bool {
        self.blocks.as_slice().eq(other.blocks())
    }
}

impl Eq for Tipset {}

impl std::hash::Hash for Tipset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.blocks.hash(state)
    }
}

impl Tipset {
    /// Create a new Tipset with the given block headers.
    pub fn new(mut blocks: Vec<BlockHeader>) -> Result<Self, TipsetError> {
        if blocks.is_empty() {
            return Err(TipsetError::EmptyBlocks);
        }

        blocks.sort_by(|i, j| {
            let ti = i.last_ticket();
            let tj = j.last_ticket();
            if ti == tj {
                // block i and block j have the same ticket, then compare the cid of the two blocks.
                i.cid().cmp(&j.cid())
            } else {
                ti.cmp(tj)
            }
        });

        let mut cids = vec![];
        let height = blocks[0].height;
        for block in &blocks {
            if block.height != height {
                return Err(TipsetError::MismatchingHeight {
                    expected: height,
                    found: block.height,
                });
            }

            for (i, cid) in block.parents.iter().enumerate() {
                if cid != &blocks[0].parents[i] {
                    return Err(TipsetError::MismatchingParent {
                        expected: blocks[0].parents[i].clone(),
                        found: cid.clone(),
                    });
                }
            }

            cids.push(block.cid());
        }

        Ok(Self {
            key: TipsetKey::new(cids),
            blocks,
            height,
        })
    }

    /// Determine whether the tipset contains a block header with a Cid value of `cid`.
    pub fn contains(&self, cid: &Cid) -> bool {
        self.key.cids().contains(cid)
    }

    /// Return the key.
    pub fn key(&self) -> &TipsetKey {
        &self.key
    }

    /// Return the CIDs of blocks.
    pub fn cids(&self) -> &[Cid] {
        self.key.cids()
    }

    /// Return the block headers.
    pub fn blocks(&self) -> &[BlockHeader] {
        &self.blocks
    }

    /// Return the height of the first block.
    pub fn height(&self) -> u64 {
        self.height
    }

    /// Return the tipset key that represents the parents of the first block.
    pub fn parents(&self) -> TipsetKey {
        TipsetKey::new((&self.blocks[0].parents).to_vec())
    }

    /// Return the parent state of the first block.
    pub fn parent_state(&self) -> &Cid {
        &self.blocks[0].parent_state_root
    }

    /// Return the parent weight of the first block.
    pub fn parent_weight(&self) -> &BigInt {
        &self.blocks[0].parent_weight
    }

    /// Return the block, which has the minimum ticket.
    pub fn min_ticket_block(&self) -> &BlockHeader {
        let mut min_block = &self.blocks[0];
        for block in &self.blocks {
            if block.last_ticket() < min_block.last_ticket() {
                min_block = block;
            }
        }
        min_block
    }

    /// Return the minimum ticket in the blocks.
    pub fn min_ticket(&self) -> &Ticket {
        &self.min_ticket_block().ticket
    }

    /// Return the minimum timestamp in the blocks.
    pub fn min_timestamp(&self) -> u64 {
        self.blocks
            .iter()
            .map(|block| block.timestamp)
            .min()
            .expect("Tipset is not empty")
    }
}

/// TipsetKey CBOR serialization/deserialization, need to use `serde_cbor::Serializer` and `serde_cbor::Deserializer`
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_block::{block_header_cbor, BlockHeader};

    use super::Tipset;
    use crate::key::TipsetKey;

    #[derive(Serialize)]
    struct CborTipsetRef<'a>(
        #[serde(with = "crate::key::cbor")] &'a TipsetKey,
        #[serde(with = "block_header_cbor::vec")] &'a [BlockHeader],
        &'a u64,
    );

    /// CBOR serialization.
    pub fn serialize<S>(tipset: &Tipset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        CborTipsetRef(&tipset.key, &tipset.blocks, &tipset.height).serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborTipset(
        #[serde(with = "crate::key::cbor")] TipsetKey,
        #[serde(with = "block_header_cbor::vec")] Vec<BlockHeader>,
        u64,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Tipset, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let CborTipset(key, blocks, height) = CborTipset::deserialize(deserializer)?;
        Ok(Tipset {
            key,
            blocks,
            height,
        })
    }
}

/// TipsetKey JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_block::{block_header_json, BlockHeader};

    use super::Tipset;
    use crate::key::TipsetKey;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonTipsetRef<'a> {
        #[serde(rename = "Cids")]
        #[serde(with = "crate::key::json")]
        key: &'a TipsetKey,
        #[serde(with = "block_header_json::vec")]
        blocks: &'a [BlockHeader],
        height: &'a u64,
    }

    /// JSON serialization.
    pub fn serialize<S>(tipset: &Tipset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonTipsetRef {
            key: &tipset.key,
            blocks: &tipset.blocks,
            height: &tipset.height,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonTipset {
        #[serde(rename = "Cids")]
        #[serde(with = "crate::key::json")]
        key: TipsetKey,
        #[serde(with = "block_header_json::vec")]
        blocks: Vec<BlockHeader>,
        height: u64,
    }

    /// JSON deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Tipset, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonTipset {
            key,
            blocks,
            height,
        } = JsonTipset::deserialize(deserializer)?;
        Ok(Tipset {
            key,
            blocks,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use cid::Cid;
    use serde::{Deserialize, Serialize};

    use plum_address::{set_network, Address, Network};
    use plum_block::{BlockHeader, ElectionProof, Ticket};
    use plum_crypto::Signature;

    use super::Tipset;
    use crate::key::TipsetKey;

    fn new_tipset() -> Tipset {
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        let block_header = BlockHeader {
            miner: Address::new_id_addr(12_512_063).unwrap(),
            ticket: Ticket {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            },
            election_proof: ElectionProof {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            },
            beacon_entries: vec![],
            win_post_proof: vec![],
            parents: vec![cid.clone(), cid.clone()],
            parent_message_receipts: cid.clone(),
            bls_aggregate: Signature::new_bls("boo! im a signature"),
            parent_weight: 123_125_126_212u64.into(),
            messages: cid.clone(),
            height: 85_919_298_723,
            parent_state_root: cid.clone(),
            timestamp: 0u64,
            block_sig: Signature::new_bls("boo! im a signature"),
            fork_signaling: 0u64,
        };

        Tipset {
            key: TipsetKey::new(vec![cid]),
            blocks: vec![block_header],
            height: 1,
        }
    }

    #[test]
    fn tipset_json_serde() {
        unsafe {
            set_network(Network::Test);
        }
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct JsonTipset(#[serde(with = "super::json")] Tipset);

        let tipset = JsonTipset(new_tipset());
        let expected = "{\
            \"Cids\":[{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"}],\
            \"Blocks\":[{\
                \"Miner\":\"t012512063\",\
                \"Ticket\":{\"VRFProof\":\"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA=\"},\
                \"ElectionProof\":{\"VRFProof\":\"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA=\"},\
                \"BeaconEntries\":[],\
                \"WinPoStProof\":[],\
                \"Parents\":[\
                    {\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"},\
                    {\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"}\
                ],\
                \"ParentWeight\":\"123125126212\",\
                \"Height\":85919298723,\
                \"ParentStateRoot\":{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"},\
                \"ParentMessageReceipts\":{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"},\
                \"Messages\":{\"/\":\"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i\"},\
                \"BLSAggregate\":{\"Type\":\"bls\",\"Data\":\"Ym9vISBpbSBhIHNpZ25hdHVyZQ==\"},\
                \"Timestamp\":0,\
                \"BlockSig\":{\"Type\":\"bls\",\"Data\":\"Ym9vISBpbSBhIHNpZ25hdHVyZQ==\"},\
                \"ForkSignaling\":0\
            }],\
            \"Height\":1\
        }";

        let ser = serde_json::to_string(&tipset).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<JsonTipset>(&ser).unwrap();
        assert_eq!(de, tipset);
    }
}
