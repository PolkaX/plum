// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{bigint::BigInt, signature::Signature};
use address::Address;
use block_format::BasicBlock;
use bytes::Bytes;
use cid::{AsCidRef, Cid, Codec, Hash, Prefix};
use core::convert::TryInto;
use log::warn;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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
        Ok(Ticket {
            vrf_proof: out.0.into_vec(),
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EPostTicket {
    pub partial: Vec<u8>,
    pub sector_id: u64,
    pub challenge_index: u64,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EPostProof {
    pub proof: Vec<u8>,
    pub post_rand: Vec<u8>,
    pub candidates: Vec<EPostTicket>,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub miner: Address,
    pub ticket: Ticket,
    pub epost_proof: EPostProof,
    pub parents: Vec<Cid>,
    pub parent_weight: BigInt,
    pub height: u64,
    pub parent_state_root: Cid,
    pub parent_message_receipts: Cid,
    pub messages: Cid,
    pub bls_aggregate: Signature,
    pub timestamp: u64,
    pub block_sig: Signature,
}

impl Ord for BlockHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_last_ticket = self.last_ticket();
        let your_last_ticket = other.last_ticket();
        match my_last_ticket.cmp(&your_last_ticket) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let my_cid = self.clone().cid();
                let your_cid = other.clone().cid();
                // FIXME: add test
                warn!("blocks have same ticket: ({} {})", self.miner, other.miner);
                println!(
                    "my_cid: bytes: {:?}, string: {:?}",
                    my_cid.to_bytes(),
                    my_cid.to_string()
                );
                println!(
                    "your_cid: bytes: {:?}, string: {:?}",
                    your_cid.to_bytes(),
                    your_cid.to_string()
                );
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
        let blk: BasicBlock = self.try_into().expect("TODO: Check this later");
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
            epost_proof: EPostProof {
                proof: b"pruuf".to_vec(),
                post_rand: b"random".to_vec(),
                candidates: Vec::new(),
            },
            ticket: Ticket {
                vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
            },
            parents: vec![cid.clone(), cid.clone()],
            parent_message_receipts: cid.clone(),
            bls_aggregate: Signature {
                ty: KeyType::BLS,
                data: b"boo! im a signature".to_vec(),
            },
            parent_weight: 123125126212,
            messages: cid.clone(),
            height: 85919298723,
            parent_state_root: cid,
            timestamp: 0u64,
            block_sig: Signature {
                ty: KeyType::BLS,
                data: b"boo! im a signature".to_vec(),
            },
        }
    }

    #[test]
    fn ticket_cbor_serialization_should_work() {
        let ticket = Ticket {
            vrf_proof: b"vrf proof0000000vrf proof0000000".to_vec(),
        };
        let expected = [
            129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
            118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
        ];
        let out = serde_cbor::to_vec(&ticket).unwrap();
        assert_eq!(out.as_slice(), &expected[..]);

        let out = serde_cbor::from_slice(&out).unwrap();
        assert_eq!(ticket, out);
    }

    #[test]
    fn epost_proof_serialization_should_work() {
        let epost_proof = EPostProof {
            proof: b"pruuf".to_vec(),
            post_rand: b"random".to_vec(),
            candidates: Vec::new(),
        };
        let expected = [
            131, 69, 112, 114, 117, 117, 102, 70, 114, 97, 110, 100, 111, 109, 128,
        ];
    }

    #[test]
    fn block_header_serialization_should_work() {
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
        let data = Bytes::from(serde_cbor::to_vec(&header).unwrap());
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
}
