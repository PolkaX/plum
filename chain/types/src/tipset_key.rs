// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::Cid;
use std::convert::TryFrom;
use thiserror::Error;

// go-cid is 32
pub const BLOCK_HEADER_CID_LEN: u8 = 38;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TipSetKey {
    value: Vec<u8>,
}

fn encode_key(cids: &[Cid]) -> Vec<u8> {
    let mut value = Vec::new();
    for cid in cids {
        value.extend(cid.to_bytes());
    }
    value
}

fn decode_key(slices: &[u8]) -> std::result::Result<Vec<Cid>, TipSetKeyError> {
    let mut cids = Vec::new();
    for chunk in slices.chunks(BLOCK_HEADER_CID_LEN as usize) {
        cids.push(Cid::from(chunk)?);
    }
    Ok(cids)
}

#[derive(Error, Debug)]
pub enum TipSetKeyError {
    #[error("cid error: {0}")]
    CidError(#[from] cid::CidError),
}

impl TipSetKey {
    pub fn new(cids: &[Cid]) -> TipSetKey {
        let value = encode_key(cids);
        Self { value }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.value
    }
}

impl TryFrom<&[u8]> for TipSetKey {
    type Error = TipSetKeyError;
    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        decode_key(bytes).and_then(|_| {
            Ok(Self {
                value: bytes.to_vec(),
            })
        })
    }
}

impl std::fmt::Display for TipSetKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cids = decode_key(&self.value).expect("failed to decode TipSetKey");
        let cids_str = cids.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        write!(f, "{{{}}}", cids_str.join(","))
    }
}

#[test]
fn new_tipset_key_should_work() {
    use std::convert::TryInto;
    let c1: Cid = "bafy2bzacect5mm5ptrpcqmrajuhmzs6tg43ytjutlsd5kjd4pvxui57er6ose"
        .parse()
        .unwrap();
    let c2: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
        .parse()
        .unwrap();

    let tsk = TipSetKey::new(&[c1, c2]);

    let expected = [
        1, 113, 160, 228, 2, 32, 167, 214, 51, 175, 156, 94, 40, 50, 32, 77, 14, 204, 203, 211, 55,
        55, 137, 166, 147, 92, 135, 213, 36, 124, 125, 111, 68, 119, 228, 143, 157, 34, 1, 113, 18,
        32, 76, 2, 122, 115, 187, 29, 97, 161, 80, 48, 167, 49, 47, 124, 18, 38, 183, 206, 50, 72,
        232, 201, 142, 225, 217, 73, 55, 160, 199, 184, 78, 250,
    ];
    assert_eq!(tsk.as_slice(), &expected[..]);

    let de: TipSetKey = expected[..].try_into().unwrap();
    assert_eq!(de, tsk);
    assert_eq!(format!("{}", de), "{bafy2bzacect5mm5ptrpcqmrajuhmzs6tg43ytjutlsd5kjd4pvxui57er6ose,bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i}");
}
