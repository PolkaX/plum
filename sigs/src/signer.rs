use anyhow::Result;

pub trait Signer {
    fn sign(&self, pubkey: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>>;
}
