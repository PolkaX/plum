// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_bigint::BigInt;
use plum_block::{BlockHeader, Ticket};

use crate::errors::TipsetError;
use crate::key::TipsetKey;

///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tipset {
    #[serde(rename = "Cids")]
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

// Implement CBOR serialization for Tipset.
impl encode::Encode for Tipset {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .encode(&self.key)?
            .encode(&self.blocks)?
            .u64(self.height)?
            .ok()
    }
}

// Implement CBOR deserialization for Tipset.
impl<'b> decode::Decode<'b> for Tipset {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(Tipset {
            key: d.decode::<TipsetKey>()?,
            blocks: d.decode::<Vec<BlockHeader>>()?,
            height: d.u64()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use cid::Cid;

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
    fn tipset_cbor_serde() {
        let tipset = new_tipset();
        let expected = vec![
            131, 129, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80,
            48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160,
            199, 184, 78, 250, 129, 143, 69, 0, 191, 214, 251, 5, 129, 88, 32, 118, 114, 102, 32,
            112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48, 118, 114, 102, 32, 112, 114, 111,
            111, 102, 48, 48, 48, 48, 48, 48, 48, 129, 88, 32, 118, 114, 102, 32, 112, 114, 111,
            111, 102, 48, 48, 48, 48, 48, 48, 48, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48,
            48, 48, 48, 48, 48, 48, 128, 128, 130, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122,
            115, 187, 29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201,
            142, 225, 217, 73, 55, 160, 199, 184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76,
            2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232,
            201, 142, 225, 217, 73, 55, 160, 199, 184, 78, 250, 70, 0, 28, 170, 212, 84, 68, 27, 0,
            0, 0, 20, 1, 48, 116, 163, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187,
            29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225,
            217, 73, 55, 160, 199, 184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122,
            115, 187, 29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201,
            142, 225, 217, 73, 55, 160, 199, 184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76,
            2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232,
            201, 142, 225, 217, 73, 55, 160, 199, 184, 78, 250, 84, 2, 98, 111, 111, 33, 32, 105,
            109, 32, 97, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 0, 84, 2, 98, 111, 111,
            33, 32, 105, 109, 32, 97, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 0, 1,
        ];
        let ser = minicbor::to_vec(&tipset).unwrap();
        assert_eq!(ser, expected);

        let de = minicbor::decode::<Tipset>(&ser).unwrap();
        assert_eq!(de, tipset);
    }

    #[test]
    fn tipset_json_serde() {
        unsafe {
            set_network(Network::Test);
        }

        let tipset = new_tipset();
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
        let de = serde_json::from_str::<Tipset>(&ser).unwrap();
        assert_eq!(de, tipset);
    }
}
