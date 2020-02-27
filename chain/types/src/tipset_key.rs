use crate::tipset::TipSet;
use cid::Cid;

pub struct TipSetKey {
    value: Vec<u8>,
}

impl TipSetKey {
    pub fn new(cids: &[Cid]) -> TipSetKey {
        let mut value = Vec::new();
        for cid in cids {
            value.extend(cid.to_bytes());
        }
        Self { value }
    }
}
