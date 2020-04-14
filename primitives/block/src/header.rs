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
    use cid::{ipld_dag_cbor as cid_cbor, Cid};
    use serde::{de, ser, Deserialize, Serialize};

    use plum_address::{address_cbor, Address};
    use plum_bigint::{bigint_cbor, BigInt};
    use plum_crypto::{signature_cbor, Signature};
    use plum_ticket::{epost_proof_cbor, ticket_cbor, EPostProof, Ticket};

    use super::BlockHeader;

    #[derive(Serialize)]
    struct CborBlockHeaderRef<'a>(
        #[serde(with = "address_cbor")] &'a Address,
        #[serde(with = "ticket_cbor")] &'a Ticket,
        #[serde(with = "epost_proof_cbor")] &'a EPostProof,
        #[serde(with = "cid_cbor::vec")] &'a [Cid],
        #[serde(with = "bigint_cbor")] &'a BigInt,
        &'a u64,
        #[serde(with = "cid_cbor")] &'a Cid,
        #[serde(with = "cid_cbor")] &'a Cid,
        #[serde(with = "cid_cbor")] &'a Cid,
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
        CborBlockHeaderRef(
            &header.miner,
            &header.ticket,
            &header.epost_proof,
            &header.parents,
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
    struct CborBlockHeader(
        #[serde(with = "address_cbor")] Address,
        #[serde(with = "ticket_cbor")] Ticket,
        #[serde(with = "epost_proof_cbor")] EPostProof,
        #[serde(with = "cid_cbor::vec")] Vec<Cid>,
        #[serde(with = "bigint_cbor")] BigInt,
        u64,
        #[serde(with = "cid_cbor")] Cid,
        #[serde(with = "cid_cbor")] Cid,
        #[serde(with = "cid_cbor")] Cid,
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
        let CborBlockHeader(
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
        ) = CborBlockHeader::deserialize(deserializer)?;
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

    /// Vec<BlockHeader> CBOR serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct CborBlockHeaderRef<'a>(#[serde(with = "super")] &'a BlockHeader);

        /// CBOR serialization of Vec<BlockHeader>.
        pub fn serialize<S>(headers: &[BlockHeader], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            headers
                .iter()
                .map(|header| CborBlockHeaderRef(header))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct CborBlockHeader(#[serde(with = "super")] BlockHeader);

        /// CBOR deserialization of Vec<BlockHeader>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BlockHeader>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let headers = <Vec<CborBlockHeader>>::deserialize(deserializer)?;
            Ok(headers
                .into_iter()
                .map(|CborBlockHeader(header)| header)
                .collect())
        }
    }
}

/// BlockHeader JSON serialization/deserialization
pub mod json {
    use cid::{ipld_dag_json as cid_json, Cid};
    use serde::{de, ser, Deserialize, Serialize};

    use plum_address::{address_json, Address};
    use plum_bigint::{bigint_json, BigInt};
    use plum_crypto::{signature_json, Signature};
    use plum_ticket::{epost_proof_json, ticket_json, EPostProof, Ticket};

    use super::BlockHeader;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockHeaderRef<'a> {
        #[serde(with = "address_json")]
        miner: &'a Address,
        #[serde(with = "ticket_json")]
        ticket: &'a Ticket,
        #[serde(rename = "EPostProof")]
        #[serde(with = "epost_proof_json")]
        epost_proof: &'a EPostProof,
        #[serde(with = "cid_json::vec")]
        parents: &'a [Cid],
        #[serde(with = "bigint_json")]
        parent_weight: &'a BigInt,
        height: &'a u64,
        #[serde(with = "cid_json")]
        parent_state_root: &'a Cid,
        #[serde(with = "cid_json")]
        parent_message_receipts: &'a Cid,
        #[serde(with = "cid_json")]
        messages: &'a Cid,
        #[serde(rename = "BLSAggregate")]
        #[serde(with = "signature_json")]
        bls_aggregate: &'a Signature,
        timestamp: &'a u64,
        #[serde(with = "signature_json")]
        block_sig: &'a Signature,
        fork_signaling: &'a u64,
    }

    /// JSON serialization
    pub fn serialize<S>(header: &BlockHeader, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonBlockHeaderRef {
            miner: &header.miner,
            ticket: &header.ticket,
            epost_proof: &header.epost_proof,
            parents: &header.parents,
            parent_weight: &header.parent_weight,
            height: &header.height,
            parent_state_root: &header.parent_state_root,
            parent_message_receipts: &header.parent_message_receipts,
            messages: &header.messages,
            bls_aggregate: &header.bls_aggregate,
            timestamp: &header.timestamp,
            block_sig: &header.block_sig,
            fork_signaling: &header.fork_signaling,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonBlockHeader {
        #[serde(with = "address_json")]
        miner: Address,
        #[serde(with = "ticket_json")]
        ticket: Ticket,
        #[serde(rename = "EPostProof")]
        #[serde(with = "epost_proof_json")]
        epost_proof: EPostProof,
        #[serde(with = "cid_json::vec")]
        parents: Vec<Cid>,
        #[serde(with = "bigint_json")]
        parent_weight: BigInt,
        height: u64,
        #[serde(with = "cid_json")]
        parent_state_root: Cid,
        #[serde(with = "cid_json")]
        parent_message_receipts: Cid,
        #[serde(with = "cid_json")]
        messages: Cid,
        #[serde(rename = "BLSAggregate")]
        #[serde(with = "signature_json")]
        bls_aggregate: Signature,
        timestamp: u64,
        #[serde(with = "signature_json")]
        block_sig: Signature,
        fork_signaling: u64,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockHeader, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let JsonBlockHeader {
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
        } = JsonBlockHeader::deserialize(deserializer)?;
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

    /// Vec<BlockHeader> JSON serialization/deserialization.
    pub mod vec {
        use super::*;

        #[derive(Serialize)]
        struct JsonBlockHeaderRef<'a>(#[serde(with = "super")] &'a BlockHeader);

        /// JSON serialization of Vec<BlockHeader>.
        pub fn serialize<S>(headers: &[BlockHeader], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            headers
                .iter()
                .map(|header| JsonBlockHeaderRef(header))
                .collect::<Vec<_>>()
                .serialize(serializer)
        }

        #[derive(Deserialize)]
        struct JsonBlockHeader(#[serde(with = "super")] BlockHeader);

        /// JSON deserialization of Vec<BlockHeader>.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BlockHeader>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            let headers = <Vec<JsonBlockHeader>>::deserialize(deserializer)?;
            Ok(headers
                .into_iter()
                .map(|JsonBlockHeader(header)| header)
                .collect())
        }
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
        let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
            .parse()
            .unwrap();

        BlockHeader {
            miner: Address::new_id_addr(12_512_063).unwrap(),
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
        let expected = r#"{"Miner":"t012512063","Ticket":{"VRFProof":"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA="},"EPostProof":{"Proof":"cHJ1dWY=","PostRand":"cmFuZG9t","Candidates":[]},"Parents":[{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"}],"ParentWeight":"123125126212","Height":85919298723,"ParentStateRoot":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"ParentMessageReceipts":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"Messages":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"BLSAggregate":{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="},"Timestamp":0,"BlockSig":{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="},"ForkSignaling":0}"#;

        let ser = serde_json::to_string(&header).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<JsonBlockHeader>(&ser).unwrap();
        assert_eq!(de, header);
    }
}
