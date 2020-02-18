// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::{
    block_header::{BlockHeader, Ticket},
    Cid,
};
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TipSetError {
    #[error("zero length array of blocks")]
    EmptyBlocks,
    #[error("mismatching heights (expected {expected:?}, found {found:?})")]
    MismatchingHeights { expected: u64, found: u64 },
    #[error("mismatching parents (expected {expected:?}, found {found:?})")]
    MismatchingParents { expected: Cid, found: Cid },
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

        let mut blks = blks;
        blks.sort();

        let height = blks[0].height;

        let mut cids = Vec::new();

        for blk in blks.iter() {
            if blk.height != height {
                return Err(TipSetError::MismatchingHeights {
                    expected: height,
                    found: blk.height,
                });
            }

            for (i, cid) in blk.parents.iter().enumerate() {
                if *cid != blks[0].parents[i] {
                    return Err(TipSetError::MismatchingParents {
                        expected: blks[0].parents[i].clone(),
                        found: cid.clone(),
                    });
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
