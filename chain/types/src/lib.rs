// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod actor;
mod ask;
mod block_header;
mod block_msg;
pub mod chain_epoch;
mod fil;
mod full_block;
pub mod key_info;
mod keystore;
mod logs;
mod message;
mod message_receipt;
mod signature;
mod signed_message;
mod tipset;
mod tipset_key;
mod vmcontext;

pub use self::actor::Actor;
pub use self::ask::{SignedStorageAsk, StorageAsk};
pub use self::block_header::{BlockHeader, EPostProof, EPostTicket, Ticket};
pub use self::block_msg::BlockMsg;
pub use self::fil::{parse_fil, FIL};
pub use self::full_block::FullBlock;
pub use self::key_info::{KeyInfo, KeyType, SignKeyType};
pub use self::keystore::KeyStore;
pub use self::logs::LogCids;
pub use self::message::Message;
pub use self::message_receipt::MessageReceipt;
pub use self::signature::{Signature, SIGNATURE_MAX_LENGTH};
pub use self::signed_message::SignedMessage;
pub use self::tipset::{TipSet, TipSetError};
pub use self::tipset_key::TipSetKey;

pub use cid::*;

pub use plum_address::*;

use block_format::{BasicBlock, Block, BlockFormatError};
use bytes::Bytes;
use cid::Codec;
use core::convert::TryInto;
use serde::Serialize;
use serde_cbor::error::Error as CborError;

#[derive(Debug, thiserror::Error)]
pub enum StorageBlockError {
    #[error("BlockFormatError: {0}")]
    BlockFormatError(#[from] BlockFormatError),
    #[error("cid error: {0}")]
    CidError(#[from] cid::Error),
    #[error("cbor err: {0}")]
    CborError(#[from] CborError),
}

pub fn to_storage_block<S: Serialize>(s: &S) -> std::result::Result<BasicBlock, StorageBlockError> {
    let data = Bytes::from(serde_cbor::to_vec(s)?);

    let hash = multihash::Blake2b256::digest(&data);
    let cid = Cid::new_v1(Codec::DagCBOR, hash);
    let block = BasicBlock::new_with_cid(data, cid)?;

    Ok(block)
}

pub fn into_cid<T: TryInto<BasicBlock, Error = StorageBlockError>>(s: T) -> Cid {
    let blk: BasicBlock = s
        .try_into()
        .expect("failed to BasicBlock, this should not happen");
    blk.cid().clone()
}
