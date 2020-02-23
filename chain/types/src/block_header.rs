// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::signature::Signature;
use address::Address;
use block_format::BasicBlock;
use bytes::Bytes;
use cid::{AsCidRef, Cid, Codec, Hash, Prefix};
use core::convert::TryInto;
use log::warn;
use rust_ipld_cbor::bigint::CborBigInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd)]
pub struct Ticket {
    pub vrf_proof: Vec<u8>,
}

impl Serialize for Ticket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = serde_bytes::Bytes::new(&self.vrf_proof);
        let to_ser = (value,);
        to_ser.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Ticket {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let out: (serde_bytes::ByteBuf,) = Deserialize::deserialize(deserializer)?;
        Ok(Self {
            vrf_proof: out.0.into_vec(),
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct EPostTicket {
    #[serde(with = "serde_bytes")]
    pub partial: Vec<u8>,
    pub sector_id: u64,
    pub challenge_index: u64,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct EPostProof {
    #[serde(with = "serde_bytes")]
    pub proof: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub post_rand: Vec<u8>,
    pub candidates: Vec<EPostTicket>,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct BlockHeader {
    pub miner: Address,
    pub ticket: Ticket,
    pub epost_proof: EPostProof,
    pub parents: Vec<Cid>,
    pub parent_weight: CborBigInt,
    pub height: u64,
    pub parent_state_root: Cid,
    pub parent_message_receipts: Cid,
    pub messages: Cid,
    pub bls_aggregate: Signature,
    pub timestamp: u64,
    pub block_sig: Signature,
    pub fork_signaling: u64,
}

impl Ord for BlockHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_last_ticket = self.last_ticket();
        let your_last_ticket = other.last_ticket();
        match my_last_ticket.cmp(&your_last_ticket) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // TODO: is clone avoidable?
                let my_cid = self.clone().cid();
                let your_cid = other.clone().cid();
                warn!("blocks have same ticket: ({} {})", self.miner, other.miner);
                my_cid.to_string().cmp(&your_cid.to_string())
            }
        }
    }
}

impl PartialOrd for BlockHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryInto<BasicBlock> for BlockHeader {
    type Error = anyhow::Error;
    fn try_into(self) -> std::result::Result<BasicBlock, Self::Error> {
        let data = Bytes::from(serde_cbor::to_vec(&self)?);

        let prefix = Prefix::new_prefix_v1(Codec::DagCBOR, Hash::Blake2b256);
        let cid = prefix.sum(&data)?;
        let block = BasicBlock::new_with_cid(data, cid)?;

        Ok(block)
    }
}

impl BlockHeader {
    pub fn cid(self) -> Cid {
        let blk: BasicBlock = self
            .try_into()
            .expect("failed to BasicBlock, this should not happen");
        blk.cid().clone()
    }

    pub fn last_ticket(&self) -> &Ticket {
        &self.ticket
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_info::KeyType;
    use address::Network;

    fn new_block_header() -> BlockHeader {
        let id = 12512063;
        let addr = address::Address::new_id_addr(Network::Test, id).unwrap();

        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        BlockHeader {
            miner: addr,
            ticket: Ticket {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            },
            epost_proof: EPostProof {
                proof: b"pruuf".to_vec(),
                post_rand: b"random".to_vec(),
                candidates: Vec::new(),
            },
            parents: vec![cid.clone(), cid.clone()],
            parent_message_receipts: cid.clone(),
            bls_aggregate: Signature {
                ty: KeyType::BLS,
                data: b"boo! im a signature".to_vec(),
            },
            parent_weight: CborBigInt(123125126212u64.into()),
            messages: cid.clone(),
            height: 85919298723,
            parent_state_root: cid,
            timestamp: 0u64,
            block_sig: Signature {
                ty: KeyType::BLS,
                data: b"boo! im a signature".to_vec(),
            },
            fork_signaling: 0u64,
        }
    }

    #[test]
    fn ticket_serde_should_work() {
        let ticket = Ticket {
            vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
        };
        let expected = [
            129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
            118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
        ];
        let ser = serde_cbor::to_vec(&ticket).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice(&ser).unwrap();
        assert_eq!(ticket, de);
    }

    #[test]
    fn epost_ticket_serde_should_work() {
        let epost_ticket = EPostTicket {
            partial: b"epost_ticket".to_vec(),
            sector_id: 6,
            challenge_index: 8,
        };
        let expected = [
            131, 76, 101, 112, 111, 115, 116, 95, 116, 105, 99, 107, 101, 116, 6, 8,
        ];
        let ser = serde_cbor::to_vec(&epost_ticket).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice(&ser).unwrap();
        assert_eq!(epost_ticket, de);
    }

    #[test]
    fn epost_proof_serde_should_work() {
        let epost_proof = EPostProof {
            proof: b"pruuf".to_vec(),
            post_rand: b"random".to_vec(),
            candidates: Vec::new(),
        };
        let expected = [
            131, 69, 112, 114, 117, 117, 102, 70, 114, 97, 110, 100, 111, 109, 128,
        ];
        let ser = serde_cbor::to_vec(&epost_proof).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice(&ser).unwrap();
        assert_eq!(epost_proof, de);
    }

    #[test]
    fn block_header_serde_should_work() {
        let header = new_block_header();
        let expected = [
            141, 69, 0, 191, 214, 251, 5, 129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102,
            48, 48, 48, 48, 48, 48, 48, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48,
            48, 48, 48, 131, 69, 112, 114, 117, 117, 102, 70, 114, 97, 110, 100, 111, 109, 128,
            130, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48,
            167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199,
            184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161,
            80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55,
            160, 199, 184, 78, 250, 70, 0, 28, 170, 212, 84, 68, 27, 0, 0, 0, 20, 1, 48, 116, 163,
            216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49,
            47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199, 184, 78,
            250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48,
            167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55, 160, 199,
            184, 78, 250, 216, 42, 88, 37, 0, 1, 113, 18, 32, 76, 2, 122, 115, 187, 29, 97, 161,
            80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72, 232, 201, 142, 225, 217, 73, 55,
            160, 199, 184, 78, 250, 84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32, 115, 105,
            103, 110, 97, 116, 117, 114, 101, 0, 84, 2, 98, 111, 111, 33, 32, 105, 109, 32, 97, 32,
            115, 105, 103, 110, 97, 116, 117, 114, 101, 0,
        ];
        let ser = serde_cbor::to_vec(&header).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice(&ser).unwrap();
        assert_eq!(header, de);
    }

    #[test]
    fn block_header_sort_should_work() {
        let mut header1 = new_block_header();
        let addr1 = address::Address::new_id_addr(Network::Main, 1).unwrap();
        header1.miner = addr1;

        let mut header2 = new_block_header();
        let addr2 = address::Address::new_id_addr(Network::Main, 2).unwrap();
        header2.miner = addr2;
        header2.ticket = Ticket {
            vrf_proof: b"vrf proof0000000vrf proof0000001".to_vec(),
        };

        let mut header3 = new_block_header();
        let addr3 = address::Address::new_id_addr(Network::Main, 3).unwrap();
        header3.miner = addr3;
        header3.ticket = Ticket {
            vrf_proof: b"vrf proof0000000vrf proof0000010".to_vec(),
        };

        let mut header4 = new_block_header();
        let addr4 = address::Address::new_id_addr(Network::Main, 4).unwrap();
        header4.miner = addr4;
        header4.ticket = Ticket {
            vrf_proof: b"vrf proof0000000vrf proof0000001".to_vec(),
        };

        let mut blks = vec![
            header2.clone(),
            header3.clone(),
            header1.clone(),
            header4.clone(),
        ];

        blks.sort();

        assert_eq!(blks, vec![header1, header2, header4, header3]);
    }

    #[test]
    fn to_storge_block_should_work() {
        let header = new_block_header();
        let storage_block: BasicBlock = header.try_into().unwrap();
        assert_eq!(
            "bafy2bzacect5mm5ptrpcqmrajuhmzs6tg43ytjutlsd5kjd4pvxui57er6ose",
            storage_block.cid().to_string()
        );
    }
}
