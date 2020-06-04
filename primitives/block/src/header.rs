// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec, IntoExt};
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use crate::beacon_entry::BeaconEntry;
use crate::election_proof::ElectionProof;
use crate::ticket::Ticket;

use plum_address::Address;
use plum_bigint::{bigint_json, BigInt, BigIntRefWrapper, BigIntWrapper};
use plum_crypto::Signature;
use plum_sector::PoStProof;
use plum_types::ChainEpoch;

/// The header part of the block.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockHeader {
    ///
    pub miner: Address,
    ///
    pub ticket: Ticket,
    ///
    pub election_proof: ElectionProof,
    ///
    pub beacon_entries: Vec<BeaconEntry>,
    ///
    #[serde(rename = "WinPoStProof")]
    pub win_post_proof: Vec<PoStProof>,
    ///
    pub parents: Vec<Cid>,
    ///
    #[serde(with = "bigint_json")]
    pub parent_weight: BigInt,
    ///
    pub height: ChainEpoch,
    ///
    pub parent_state_root: Cid,
    ///
    pub parent_message_receipts: Cid,
    ///
    pub messages: Cid,
    ///
    #[serde(rename = "BLSAggregate")]
    pub bls_aggregate: Signature,
    ///
    pub timestamp: u64,
    ///
    pub block_sig: Signature,
    ///
    pub fork_signaling: u64,

    /*
    /// internal
    #[serde(skip)]
    validated: bool, // true if the signature has been validated
    */
}

impl BlockHeader {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        let data =
            minicbor::to_vec(self).expect("CBOR serialization of BlockHeader shouldn't be failed");
        self.cid_with_data(data)
    }

    /// Convert to the CID with the given CBOR serialized data of BlockHeader.
    ///
    /// For cases where serialized data of the BlockHeader is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        let hash = multihash::Blake2b256::digest(data.as_ref()).into_ext();
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    ///
    pub fn last_ticket(&self) -> &Ticket {
        &self.ticket
    }
}

// Implement CBOR serialization for BlockHeader.
impl encode::Encode for BlockHeader {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(15)?
            .encode(&self.miner)?
            .encode(&self.ticket)?
            .encode(&self.election_proof)?
            .encode(&self.beacon_entries)?
            .encode(&self.win_post_proof)?
            .encode(&self.parents)?
            .encode(BigIntRefWrapper::from(&self.parent_weight))?
            .i64(self.height)?
            .encode(&self.parent_state_root)?
            .encode(&self.parent_message_receipts)?
            .encode(&self.messages)?
            .encode(&self.bls_aggregate)?
            .u64(self.timestamp)?
            .encode(&self.block_sig)?
            .u64(self.fork_signaling)?
            .ok()
    }
}

// Implement CBOR deserialization for BlockHeader.
impl<'b> decode::Decode<'b> for BlockHeader {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(15));
        Ok(BlockHeader {
            miner: d.decode::<Address>()?,
            ticket: d.decode::<Ticket>()?,
            election_proof: d.decode::<ElectionProof>()?,
            beacon_entries: d.decode::<Vec<BeaconEntry>>()?,
            win_post_proof: d.decode::<Vec<PoStProof>>()?,
            parents: d.decode::<Vec<Cid>>()?,
            parent_weight: d.decode::<BigIntWrapper>()?.into_inner(),
            height: d.i64()?,
            parent_state_root: d.decode::<Cid>()?,
            parent_message_receipts: d.decode::<Cid>()?,
            messages: d.decode::<Cid>()?,
            bls_aggregate: d.decode::<Signature>()?,
            timestamp: d.u64()?,
            block_sig: d.decode::<Signature>()?,
            fork_signaling: d.u64()?,
            /*validated: Default::default(),*/
        })
    }
}

#[cfg(test)]
mod tests {
    use cid::Cid;

    use plum_address::{set_network, Address, Network};
    use plum_crypto::Signature;

    use super::BlockHeader;
    use crate::election_proof::ElectionProof;
    use crate::ticket::Ticket;

    pub fn dummy_block_header() -> BlockHeader {
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        BlockHeader {
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
            parent_state_root: cid,
            timestamp: 0u64,
            block_sig: Signature::new_bls("boo! im a signature"),
            fork_signaling: 0u64,
            /*validated: false,*/
        }
    }

    #[test]
    fn block_header_cbor_serde() {
        let header = dummy_block_header();
        let expected = vec![
            143, 69, 0, 191, 214, 251, 5, 129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102,
            48, 48, 48, 48, 48, 48, 48, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48,
            48, 48, 48, 129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48,
            48, 48, 48, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
            128, 128, 130, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161,
            80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55,
            160, 199, 184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29,
            97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217,
            73, 55, 160, 199, 184, 78, 250, 70, 0, 28, 170, 212, 84, 68, 27, 0, 0, 0, 20, 1, 48,
            116, 163, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80,
            48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160,
            199, 184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97,
            161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73,
            55, 160, 199, 184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187,
            29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225,
            217, 73, 55, 160, 199, 184, 78, 250, 84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32,
            115, 105, 103, 110, 97, 116, 117, 114, 101, 0, 84, 2, 98, 111, 111, 33, 32, 105, 109,
            32, 97, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 0,
        ];

        let ser = minicbor::to_vec(&header).unwrap();
        assert_eq!(ser, expected);
        let de = minicbor::decode::<BlockHeader>(&ser).unwrap();
        assert_eq!(de, header);
    }

    #[test]
    fn block_header_json_serde() {
        unsafe {
            set_network(Network::Test);
        }
        let header = dummy_block_header();
        let expected = "{\
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
            }";
        let ser = serde_json::to_string(&header).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<BlockHeader>(&ser).unwrap();
        assert_eq!(de, header);
    }
}
