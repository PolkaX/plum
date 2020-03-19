// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use cid::Cid;

use plum_block::BlockHeader;

use crate::key::TipsetKey;

///
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Tipset {
    key: TipsetKey,
    blocks: Vec<BlockHeader>,
    height: u64,
}

impl Tipset {
    ///
    pub fn new(key: TipsetKey, blocks: Vec<BlockHeader>) -> Self {
        todo!()
    }

    /// Return the key.
    pub fn key(&self) -> &TipsetKey {
        &self.key
    }

    /// Return the CIDs of blocks.
    pub fn cids(&self) -> &[Cid] {
        self.key.cids()
    }

    ///
    pub fn blocks(&self) -> &[BlockHeader] {
        &self.blocks
    }

    ///
    pub fn height(&self) -> u64 {
        self.height
    }
}

/// TipsetKey CBOR serialization/deserialization, need to use `serde_cbor::Serializer` and `serde_cbor::Deserializer`
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_block::{block_header_cbor, BlockHeader};

    use super::Tipset;
    use crate::key::TipsetKey;

    #[derive(Serialize)]
    struct CborBlockHeaderRef<'a>(#[serde(with = "block_header_cbor")] &'a BlockHeader);
    #[derive(Serialize)]
    struct TupleTipsetRef<'a>(
        #[serde(with = "crate::key::cbor")] &'a TipsetKey,
        &'a [CborBlockHeaderRef<'a>],
        &'a u64,
    );

    /// CBOR serialization.
    pub fn serialize<S>(tipset: &Tipset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        TupleTipsetRef(
            &tipset.key,
            &tipset
                .blocks
                .iter()
                .map(|block| CborBlockHeaderRef(block))
                .collect::<Vec<_>>(),
            &tipset.height,
        )
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct CborBlockHeader(#[serde(with = "block_header_cbor")] BlockHeader);
    #[derive(Deserialize)]
    struct TupleTipset(
        #[serde(with = "crate::key::cbor")] TipsetKey,
        Vec<CborBlockHeader>,
        u64,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Tipset, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleTipset(key, blocks, height) = TupleTipset::deserialize(deserializer)?;
        let blocks = blocks.into_iter().map(|block| block.0).collect();
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
    struct JsonBlockHeaderRef<'a>(#[serde(with = "block_header_json")] &'a BlockHeader);
    #[derive(Serialize)]
    struct JsonTipsetRef<'a> {
        #[serde(rename = "Cids")]
        #[serde(with = "crate::key::json")]
        key: &'a TipsetKey,
        #[serde(rename = "Blocks")]
        blocks: &'a [JsonBlockHeaderRef<'a>],
        #[serde(rename = "Height")]
        height: &'a u64,
    }

    /// JSON serialization.
    pub fn serialize<S>(tipset: &Tipset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        JsonTipsetRef {
            key: &tipset.key,
            blocks: &tipset
                .blocks
                .iter()
                .map(|block| JsonBlockHeaderRef(block))
                .collect::<Vec<_>>(),
            height: &tipset.height,
        }
        .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct JsonBlockHeader(#[serde(with = "block_header_json")] BlockHeader);
    #[derive(Deserialize)]
    struct JsonTipset {
        #[serde(rename = "Cids")]
        #[serde(with = "crate::key::json")]
        key: TipsetKey,
        #[serde(rename = "Blocks")]
        blocks: Vec<JsonBlockHeader>,
        #[serde(rename = "Height")]
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
            blocks: blocks.into_iter().map(|block| block.0).collect(),
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use cid::Cid;
    use serde::{Deserialize, Serialize};

    use plum_address::{set_network, Address, Network};
    use plum_block::BlockHeader;
    use plum_crypto::Signature;
    use plum_ticket::{EPostProof, Ticket};

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
        let expected = r#"{"Cids":[{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"}],"Blocks":[{"Miner":"t012512063","Ticket":{"VRFProof":"dnJmIHByb29mMDAwMDAwMHZyZiBwcm9vZjAwMDAwMDA="},"EPostProof":{"Proof":"cHJ1dWY=","PostRand":"cmFuZG9t","Candidates":[]},"Parents":[{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"}],"ParentWeight":"123125126212","Height":85919298723,"ParentStateRoot":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"ParentMessageReceipts":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"Messages":{"/":"bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"},"BLSAggregate":{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="},"Timestamp":0,"BlockSig":{"Type":"bls","Data":"Ym9vISBpbSBhIHNpZ25hdHVyZQ=="},"ForkSignaling":0}],"Height":1}"#;

        let ser = serde_json::to_string(&tipset).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<JsonTipset>(&ser).unwrap();
        assert_eq!(de, tipset);
    }
}
