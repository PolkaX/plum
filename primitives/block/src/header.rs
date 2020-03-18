// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::{Cid, Codec};
use serde::{de, ser};

use plum_address::Address;
use plum_bigint::BigInt;
use plum_crypto::Signature;
use plum_ticket::{EPostProof, Ticket};

/// The header part of the block.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct BlockHeader {
    ///
    pub miner: Address,
    ///
    pub ticket: Ticket,
    ///
    pub epost_proof: EPostProof,
    ///
    pub parents: Vec<Cid>,
    ///
    pub parent_weight: BigInt,
    ///
    pub height: u64,
    ///
    pub parent_state_root: Cid,
    ///
    pub parent_message_receipts: Cid,
    ///
    pub messages: Cid,
    ///
    pub bls_aggregate: Signature,
    ///
    pub timestamp: u64,
    ///
    pub block_sig: Signature,
    ///
    pub fork_signaling: u64,
}

impl BlockHeader {
    /// Convert to the CID.
    pub fn cid(&self) -> Cid {
        let data = serde_cbor::to_vec(self)
            .expect("CBOR serialization of BlockHeader shouldn't be failed");
        self.cid_with_data(data)
    }

    /// Convert to the CID with the given CBOR serialized data of BlockHeader.
    ///
    /// For cases where serialized data of the BlockHeader is already known,
    /// it's more cheaper than `cid`.
    pub fn cid_with_data(&self, data: impl AsRef<[u8]>) -> Cid {
        let hash = multihash::Blake2b256::digest(data.as_ref());
        Cid::new_v1(Codec::DagCBOR, hash)
    }

    ///
    pub fn last_ticket(&self) -> &Ticket {
        &self.ticket
    }
}

impl ser::Serialize for BlockHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for BlockHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// BlockHeader CBOR serialization/deserialization
pub mod cbor {
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

    use plum_address::{address_cbor, Address};
    use plum_bigint::{bigint_cbor, BigInt};
    use plum_crypto::{signature_cbor, Signature};
    use plum_ticket::{epost_proof_cbor, ticket_cbor, EPostProof, Ticket};

    use super::BlockHeader;

    #[derive(Serialize)]
    struct CborCidRef<'a>(#[serde(with = "cid::ipld_dag_cbor")] &'a Cid);
    #[derive(Serialize)]
    struct TupleBlockHeaderRef<'a>(
        #[serde(with = "address_cbor")] &'a Address,
        #[serde(with = "ticket_cbor")] &'a Ticket,
        #[serde(with = "epost_proof_cbor")] &'a EPostProof,
        &'a [CborCidRef<'a>],
        #[serde(with = "bigint_cbor")] &'a BigInt,
        &'a u64,
        #[serde(with = "cid::ipld_dag_cbor")] &'a Cid,
        #[serde(with = "cid::ipld_dag_cbor")] &'a Cid,
        #[serde(with = "cid::ipld_dag_cbor")] &'a Cid,
        #[serde(with = "signature_cbor")] &'a Signature,
        &'a u64,
        #[serde(with = "signature_cbor")] &'a Signature,
        &'a u64,
    );

    /// CBOR serialization
    pub fn serialize<S>(header: &BlockHeader, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let parents = header
            .parents
            .iter()
            .map(|parent| CborCidRef(parent))
            .collect::<Vec<_>>();
        TupleBlockHeaderRef(
            &header.miner,
            &header.ticket,
            &header.epost_proof,
            &parents,
            &header.parent_weight,
            &header.height,
            &header.parent_state_root,
            &header.parent_message_receipts,
            &header.messages,
            &header.bls_aggregate,
            &header.timestamp,
            &header.block_sig,
            &header.fork_signaling,
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborCid(#[serde(with = "cid::ipld_dag_cbor")] Cid);
    #[derive(Deserialize)]
    struct TupleBlockHeader(
        #[serde(with = "address_cbor")] Address,
        #[serde(with = "ticket_cbor")] Ticket,
        #[serde(with = "epost_proof_cbor")] EPostProof,
        Vec<CborCid>,
        #[serde(with = "bigint_cbor")] BigInt,
        u64,
        #[serde(with = "cid::ipld_dag_cbor")] Cid,
        #[serde(with = "cid::ipld_dag_cbor")] Cid,
        #[serde(with = "cid::ipld_dag_cbor")] Cid,
        #[serde(with = "signature_cbor")] Signature,
        u64,
        #[serde(with = "signature_cbor")] Signature,
        u64,
    );

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockHeader, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleBlockHeader(
            miner,
            ticket,
            epost_proof,
            parents,
            parent_weight,
            height,
            parent_state_root,
            parent_message_receipts,
            messages,
            bls_aggregate,
            timestamp,
            block_sig,
            fork_signaling,
        ) = TupleBlockHeader::deserialize(deserializer)?;
        let parents = parents.into_iter().map(|parent| parent.0).collect();
        Ok(BlockHeader {
            miner,
            ticket,
            epost_proof,
            parents,
            parent_weight,
            height,
            parent_state_root,
            parent_message_receipts,
            messages,
            bls_aggregate,
            timestamp,
            block_sig,
            fork_signaling,
        })
    }
}

/// BlockHeader JSON serialization/deserialization
pub mod json {
    use cid::Cid;
    use serde::{de, ser, Deserialize, Serialize};

    use plum_address::{address_json, Address};
    use plum_bigint::{bigint_json, BigInt};
    use plum_crypto::{signature_json, Signature};
    use plum_ticket::{epost_proof_json, ticket_json, EPostProof, Ticket};

    use super::BlockHeader;

    #[derive(Serialize)]
    struct JsonCidRef<'a>(#[serde(with = "cid::ipld_dag_json")] &'a Cid);
    #[derive(Serialize)]
    struct JsonBlockHeaderRef<'a> {
        #[serde(rename = "Miner")]
        #[serde(with = "address_json")]
        miner: &'a Address,
        #[serde(rename = "Ticket")]
        #[serde(with = "ticket_json")]
        ticket: &'a Ticket,
        #[serde(rename = "EPostProof")]
        #[serde(with = "epost_proof_json")]
        epost_proof: &'a EPostProof,
        #[serde(rename = "Parents")]
        parents: &'a [JsonCidRef<'a>],
        #[serde(rename = "ParentWeight")]
        #[serde(with = "bigint_json")]
        parent_weight: &'a BigInt,
        #[serde(rename = "Height")]
        height: &'a u64,
        #[serde(rename = "ParentStateRoot")]
        parent_state_root: &'a JsonCidRef<'a>,
        #[serde(rename = "ParentMessageReceipts")]
        parent_message_receipts: &'a JsonCidRef<'a>,
        #[serde(rename = "Messages")]
        messages: &'a JsonCidRef<'a>,
        #[serde(rename = "BLSAggregate")]
        #[serde(with = "signature_json")]
        bls_aggregate: &'a Signature,
        #[serde(rename = "Timestamp")]
        timestamp: &'a u64,
        #[serde(rename = "BlockSig")]
        #[serde(with = "signature_json")]
        block_sig: &'a Signature,
        #[serde(rename = "ForkSignaling")]
        fork_signaling: &'a u64,
    }

    /// JSON serialization
    pub fn serialize<S>(header: &BlockHeader, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let parents = header
            .parents
            .iter()
            .map(|parent| JsonCidRef(parent))
            .collect::<Vec<_>>();
        JsonBlockHeaderRef {
            miner: &header.miner,
            ticket: &header.ticket,
            epost_proof: &header.epost_proof,
            parents: &parents,
            parent_weight: &header.parent_weight,
            height: &header.height,
            parent_state_root: &JsonCidRef(&header.parent_state_root),
            parent_message_receipts: &JsonCidRef(&header.parent_message_receipts),
            messages: &JsonCidRef(&header.messages),
            bls_aggregate: &header.bls_aggregate,
            timestamp: &header.timestamp,
            block_sig: &header.block_sig,
            fork_signaling: &header.fork_signaling,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonCid(#[serde(with = "cid::ipld_dag_json")] Cid);
    #[derive(Deserialize)]
    struct JsonBlockHeader {
        #[serde(rename = "Miner")]
        #[serde(with = "address_json")]
        miner: Address,
        #[serde(rename = "Ticket")]
        #[serde(with = "ticket_json")]
        ticket: Ticket,
        #[serde(rename = "EPostProof")]
        #[serde(with = "epost_proof_json")]
        epost_proof: EPostProof,
        #[serde(rename = "Parents")]
        parents: Vec<JsonCid>,
        #[serde(rename = "ParentWeight")]
        #[serde(with = "bigint_json")]
        parent_weight: BigInt,
        #[serde(rename = "Height")]
        height: u64,
        #[serde(rename = "ParentStateRoot")]
        parent_state_root: JsonCid,
        #[serde(rename = "ParentMessageReceipts")]
        parent_message_receipts: JsonCid,
        #[serde(rename = "Messages")]
        messages: JsonCid,
        #[serde(rename = "BLSAggregate")]
        #[serde(with = "signature_json")]
        bls_aggregate: Signature,
        #[serde(rename = "Timestamp")]
        timestamp: u64,
        #[serde(rename = "BlockSig")]
        #[serde(with = "signature_json")]
        block_sig: Signature,
        #[serde(rename = "ForkSignaling")]
        fork_signaling: u64,
    }

    /// CBOR deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockHeader, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let header = JsonBlockHeader::deserialize(deserializer)?;
        let parents = header.parents.into_iter().map(|parent| parent.0).collect();
        Ok(BlockHeader {
            miner: header.miner,
            ticket: header.ticket,
            epost_proof: header.epost_proof,
            parents,
            parent_weight: header.parent_weight,
            height: header.height,
            parent_state_root: header.parent_state_root.0,
            parent_message_receipts: header.parent_message_receipts.0,
            messages: header.messages.0,
            bls_aggregate: header.bls_aggregate,
            timestamp: header.timestamp,
            block_sig: header.block_sig,
            fork_signaling: header.fork_signaling,
        })
    }
}

#[cfg(test)]
mod tests {
    use cid::Cid;
    use serde::{Deserialize, Serialize};

    use plum_address::{set_network, Address, Network};
    use plum_crypto::Signature;
    use plum_ticket::{EPostProof, Ticket};

    use super::BlockHeader;

    fn new_block_header() -> BlockHeader {
        let id = 12_512_063;
        let addr = Address::new_id_addr(id).unwrap();

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
                candidates: vec![],
            },
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
        }
    }

    #[test]
    fn block_header_cbor_serde() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct CborBlockHeader(#[serde(with = "super::cbor")] BlockHeader);

        let header = CborBlockHeader(new_block_header());
        let expected = vec![
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
        assert_eq!(ser, expected);
        let de = serde_cbor::from_slice::<CborBlockHeader>(&ser).unwrap();
        assert_eq!(de, header);
    }

    #[test]
    fn block_header_json_serde() {
        unsafe {
            set_network(Network::Test);
        }
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct JsonBlockHeader(#[serde(with = "super::json")] BlockHeader);

        let header = JsonBlockHeader(new_block_header());
        let expected_bytes = vec![
            123, 34, 77, 105, 110, 101, 114, 34, 58, 34, 116, 48, 49, 50, 53, 49, 50, 48, 54, 51,
            34, 44, 34, 84, 105, 99, 107, 101, 116, 34, 58, 123, 34, 86, 82, 70, 80, 114, 111, 111,
            102, 34, 58, 34, 100, 110, 74, 109, 73, 72, 66, 121, 98, 50, 57, 109, 77, 68, 65, 119,
            77, 68, 65, 119, 77, 72, 90, 121, 90, 105, 66, 119, 99, 109, 57, 118, 90, 106, 65, 119,
            77, 68, 65, 119, 77, 68, 65, 61, 34, 125, 44, 34, 69, 80, 111, 115, 116, 80, 114, 111,
            111, 102, 34, 58, 123, 34, 80, 114, 111, 111, 102, 34, 58, 34, 99, 72, 74, 49, 100, 87,
            89, 61, 34, 44, 34, 80, 111, 115, 116, 82, 97, 110, 100, 34, 58, 34, 99, 109, 70, 117,
            90, 71, 57, 116, 34, 44, 34, 67, 97, 110, 100, 105, 100, 97, 116, 101, 115, 34, 58, 91,
            93, 125, 44, 34, 80, 97, 114, 101, 110, 116, 115, 34, 58, 91, 123, 34, 47, 34, 58, 34,
            98, 97, 102, 121, 114, 101, 105, 99, 109, 97, 106, 53, 104, 104, 111, 121, 53, 109,
            103, 113, 118, 97, 109, 102, 104, 103, 101, 120, 120, 121, 101, 114, 103, 119, 55, 104,
            100, 101, 115, 104, 105, 122, 103, 104, 111, 100, 119, 107, 106, 103, 54, 113, 109,
            112, 111, 99, 111, 55, 105, 34, 125, 44, 123, 34, 47, 34, 58, 34, 98, 97, 102, 121,
            114, 101, 105, 99, 109, 97, 106, 53, 104, 104, 111, 121, 53, 109, 103, 113, 118, 97,
            109, 102, 104, 103, 101, 120, 120, 121, 101, 114, 103, 119, 55, 104, 100, 101, 115,
            104, 105, 122, 103, 104, 111, 100, 119, 107, 106, 103, 54, 113, 109, 112, 111, 99, 111,
            55, 105, 34, 125, 93, 44, 34, 80, 97, 114, 101, 110, 116, 87, 101, 105, 103, 104, 116,
            34, 58, 34, 49, 50, 51, 49, 50, 53, 49, 50, 54, 50, 49, 50, 34, 44, 34, 72, 101, 105,
            103, 104, 116, 34, 58, 56, 53, 57, 49, 57, 50, 57, 56, 55, 50, 51, 44, 34, 80, 97, 114,
            101, 110, 116, 83, 116, 97, 116, 101, 82, 111, 111, 116, 34, 58, 123, 34, 47, 34, 58,
            34, 98, 97, 102, 121, 114, 101, 105, 99, 109, 97, 106, 53, 104, 104, 111, 121, 53, 109,
            103, 113, 118, 97, 109, 102, 104, 103, 101, 120, 120, 121, 101, 114, 103, 119, 55, 104,
            100, 101, 115, 104, 105, 122, 103, 104, 111, 100, 119, 107, 106, 103, 54, 113, 109,
            112, 111, 99, 111, 55, 105, 34, 125, 44, 34, 80, 97, 114, 101, 110, 116, 77, 101, 115,
            115, 97, 103, 101, 82, 101, 99, 101, 105, 112, 116, 115, 34, 58, 123, 34, 47, 34, 58,
            34, 98, 97, 102, 121, 114, 101, 105, 99, 109, 97, 106, 53, 104, 104, 111, 121, 53, 109,
            103, 113, 118, 97, 109, 102, 104, 103, 101, 120, 120, 121, 101, 114, 103, 119, 55, 104,
            100, 101, 115, 104, 105, 122, 103, 104, 111, 100, 119, 107, 106, 103, 54, 113, 109,
            112, 111, 99, 111, 55, 105, 34, 125, 44, 34, 77, 101, 115, 115, 97, 103, 101, 115, 34,
            58, 123, 34, 47, 34, 58, 34, 98, 97, 102, 121, 114, 101, 105, 99, 109, 97, 106, 53,
            104, 104, 111, 121, 53, 109, 103, 113, 118, 97, 109, 102, 104, 103, 101, 120, 120, 121,
            101, 114, 103, 119, 55, 104, 100, 101, 115, 104, 105, 122, 103, 104, 111, 100, 119,
            107, 106, 103, 54, 113, 109, 112, 111, 99, 111, 55, 105, 34, 125, 44, 34, 66, 76, 83,
            65, 103, 103, 114, 101, 103, 97, 116, 101, 34, 58, 123, 34, 84, 121, 112, 101, 34, 58,
            34, 98, 108, 115, 34, 44, 34, 68, 97, 116, 97, 34, 58, 34, 89, 109, 57, 118, 73, 83,
            66, 112, 98, 83, 66, 104, 73, 72, 78, 112, 90, 50, 53, 104, 100, 72, 86, 121, 90, 81,
            61, 61, 34, 125, 44, 34, 84, 105, 109, 101, 115, 116, 97, 109, 112, 34, 58, 48, 44, 34,
            66, 108, 111, 99, 107, 83, 105, 103, 34, 58, 123, 34, 84, 121, 112, 101, 34, 58, 34,
            98, 108, 115, 34, 44, 34, 68, 97, 116, 97, 34, 58, 34, 89, 109, 57, 118, 73, 83, 66,
            112, 98, 83, 66, 104, 73, 72, 78, 112, 90, 50, 53, 104, 100, 72, 86, 121, 90, 81, 61,
            61, 34, 125, 44, 34, 70, 111, 114, 107, 83, 105, 103, 110, 97, 108, 105, 110, 103, 34,
            58, 48, 125,
        ];
        let expected_str = r#"{"Miner":"t012512063","Ticket":{"VRFProof":"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA="},"EPostProof":{"Proof":"cHJ1dWY=","PostRand":"cmFuZG9t","Candidates":[]},"Parents":[{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"}],"ParentWeight":"123125126212","Height":85919298723,"ParentStateRoot":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"ParentMessageReceipts":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"Messages":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"BLSAggregate":{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="},"Timestamp":0,"BlockSig":{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="},"ForkSignaling":0}"#;

        let ser = serde_json::to_vec(&header).unwrap();
        assert_eq!(ser, expected_bytes);
        let de = serde_json::from_slice::<JsonBlockHeader>(&ser).unwrap();
        assert_eq!(de, header);

        let ser = serde_json::to_string(&header).unwrap();
        assert_eq!(ser, expected_str);
        let de = serde_json::from_str::<JsonBlockHeader>(&ser).unwrap();
        assert_eq!(de, header);
    }
}
