// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use crate::{
    block_header::{BlockHeader, Ticket},
    Cid,
};
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TipSetError {
    #[error("Cannot create TipSet with zero length array of blocks")]
    EmptyBlocks,
    #[error("Cannot create TipSet with mismatching heights")]
    MismatchingHeights,
    #[error("Cannot create TipSet with mismatching parents")]
    MismatchingParents,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TipSet {
    cids: Vec<Cid>,
    blks: Vec<BlockHeader>,
    height: u64,
}

impl TipSet {
    pub fn new(blks: Vec<BlockHeader>) -> Result<Self, TipSetError> {
        if blks.is_empty() {
            return Err(TipSetError::EmptyBlocks);
        }

        // TODO:
        // sort.Slice(blks, tipsetSortFunc(blks))

        let height = blks[0].height;

        let mut cids = Vec::new();

        for blk in blks.iter() {
            if blk.height != height {
                return Err(TipSetError::MismatchingHeights);
            }

            for (i, cid) in blk.parents.iter().enumerate() {
                if *cid != blks[0].parents[i] {
                    return Err(TipSetError::MismatchingParents);
                }
                cids.push(blk.clone().cid());
            }
        }

        Ok(Self { cids, blks, height })
    }

    pub fn cids(&self) -> &[Cid] {
        &self.cids
    }

    pub fn blks(&self) -> &[BlockHeader] {
        &self.blks
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn min_ticket(&self) -> &Ticket {
        &self.min_ticket_block().ticket
    }

    pub fn min_ticket_block(&self) -> &BlockHeader {
        &self.blks[0]
    }

    pub fn min_timestamp(&self) -> u64 {
        self.blks
            .iter()
            .map(|blk| blk.timestamp)
            .min()
            .expect("Each created TipSet has non-empty blks; qed")
    }
}
