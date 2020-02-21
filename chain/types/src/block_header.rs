// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{bigint::BigInt, signature::Signature};
use address::Address;
use block_format::BasicBlock;
use bytes::Bytes;
use cid::{AsCidRef, Cid, Codec, Hash, Prefix};
use core::convert::TryInto;
use log::warn;
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Deserialize)]
pub struct Ticket {
    pub vrf_proof: Vec<u8>,
}

mod my_serde {
    use serde::private::ser::Error;
    use serde::ser::{SerializeSeq, Serializer};

    pub struct MySerializer;

    impl Serializer for MySerializer {
        type Ok = ();
        type Error = Error;

        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            let mut seq = self.serialize_seq(Some(v.len()))?;
            for b in v {
                seq.serialize_element(b)?;
            }
            seq.end()
        }

        serde::__serialize_unimplemented! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str none some
            unit unit_struct unit_variant newtype_struct newtype_variant
            seq tuple tuple_struct tuple_variant map struct struct_variant
        }
    }
}

// Ticket
//
// vrf_proof 为 Vec<u8>
//
// rust:
// 129 + cbor_encode_major_type() + vrf_proof
// 129 + [88, 32] + vrf_proof
//
// serializer.serialize_bytes(&bytes)
// [88, 35] + 129 + [88, 32] + vrf_proof
//
// go:
// 129 + cbor_encode_major_type() + vrf_proof
// 129 + [88, 32] + vrf_proof

impl Serialize for Ticket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // lotus/chain/types/cbor_gen.go
        let mut bytes = vec![];
        bytes.push(129u8);
        bytes.extend_from_slice(&crate::cbor_gen::cbor_encode_major_type(
            crate::cbor_gen::MajByteString,
            self.vrf_proof.len() as u64,
        ));
        bytes.extend_from_slice(&self.vrf_proof);

        let value = serde_bytes::Bytes::new(&bytes);
        serializer.serialize_bytes(&value)
    }
}

/*
impl Serialize for Ticket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // lotus/chain/types/cbor_gen.go
        let mut bytes = vec![];
        bytes.push(129u8);
        bytes.extend_from_slice(&crate::cbor_gen::cbor_encode_major_type(
            crate::cbor_gen::MajByteString,
            self.vrf_proof.len() as u64,
        ));
        bytes.extend_from_slice(&self.vrf_proof);
        println!("bytes: {:?}", bytes);
        serializer.serialize_bytes(&bytes)
    }
}
*/

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
        println!("data: {:?}", data);

        let prefix = Prefix::new_prefix_v1(Codec::DagCBOR, Hash::Blake2b256);
        let cid = prefix.sum(&data)?;
        let block = BasicBlock::new_with_cid(data, cid)?;

        Ok(block)
    }
}

impl BlockHeader {
    pub fn cid(self) -> Cid {
        let blk: BasicBlock = self.try_into().expect("TODO: Check this later");
        println!("blk: {}", blk);
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
        println!(
            "ticket vrf_proof: {:?}",
            b"vrf proof0000000vrf proof0000000".to_vec()
        );
        let expected = [
            129, 88, 32, 118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
            118, 114, 102, 32, 112, 114, 111, 111, 102, 48, 48, 48, 48, 48, 48, 48,
        ];
        println!("expected: {:?}", &expected[..]);
        println!("ticket: {:?}", serde_cbor::to_vec(&ticket).unwrap());
    }

    #[test]
    fn block_header_serialization_should_work() {
        let header = new_block_header();
        let data = Bytes::from(serde_cbor::to_vec(&header).unwrap());
        println!("data: {:?}", serde_cbor::to_vec(&header).unwrap());

        println!(
            "miner addr: {:?}",
            serde_cbor::to_vec(&header.miner).unwrap()
        );
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
        // println!("blks: {:#?}", blks);
    }

    #[test]
    fn cbor_encode_test() {
        use crate::cbor_gen::cbor_encode_major_type;
        use crate::cbor_gen::MajByteString;

        let addr_bytes = [0, 191, 214, 251, 5];
        let encoded = cbor_encode_major_type(MajByteString, addr_bytes.len() as u64);
        println!("cbor_encode_major_type: {:?}", encoded);

        let encoded = cbor_encode_major_type(MajByteString, 32u64);
        assert_eq!(encoded, [88, 32]);

        let encoded = cbor_encode_major_type(MajByteString, 12345u64);
        assert_eq!(encoded, [89, 48, 57]);

        let encoded = cbor_encode_major_type(MajByteString, 123456u64);
        assert_eq!(encoded, [90, 0, 1, 226, 64]);

        let test_u64 = 4294967295u64 + 666u64;
        let encoded = cbor_encode_major_type(MajByteString, test_u64);
        assert_eq!(encoded, [91, 0, 0, 0, 1, 0, 0, 2, 153]);
    }
}
