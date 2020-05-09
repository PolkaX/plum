// Copyright 2020 PolkaX

use anyhow::Result;
use cid::Cid;

use plum_actor::abi::piece::{PieceInfo, UnpaddedPieceSize};
use plum_actor::abi::sector::{ActorId, PoStProof, Randomness, SectorId, SectorInfo};

pub trait Storage<R: std::io::Read> {
    fn new_sector(sector: SectorId) -> Result<()>;
    fn add_piece(
        sector: SectorId,
        piece_siezes: UnpaddedPieceSize,
        new_piece_size: UnpaddedPieceSize,
        piece_data: std::io::BufReader<R>,
    ) -> Result<PieceInfo>;
}

pub trait Prover {
    fn generate_winning_post(
        miner_id: ActorId,
        sector_info: &[SectorInfo],
        randomness: Randomness,
    ) -> Result<PoStProof>;
    fn generate_window_post(
        miner_id: ActorId,
        sector_info: &[SectorInfo],
        randomness: Randomness,
    ) -> Result<PoStProof>;
}

pub type PreCommit1Out = Vec<u8>;
pub type Commit1Out = Vec<u8>;
pub type Proof = Vec<u8>;
pub type InteractiveSealRandomness = Randomness;
pub type SealRandomness = Randomness;

pub struct SectorCids {
    pub unsealed: Cid,
    pub sealed: Cid,
}

pub trait Sealer {
    fn seal_pre_commit1(
        sector: SectorId,
        ticket: SealRandomness,
        pieces: PieceInfo,
    ) -> Result<PreCommit1Out>;
    fn seal_pre_commit2(sector: SectorId, pc1o: PreCommit1Out) -> Result<SectorCids>;
    fn seal_commit1(
        sector: SectorId,
        ticket: SealRandomness,
        seed: InteractiveSealRandomness,
        pieces: &[PieceInfo],
        cids: SectorCids,
    ) -> Result<Commit1Out>;
    fn seal_commit2(sector: SectorId, c1o: Commit1Out) -> Result<Proof>;
    fn finalize_sector(sector: SectorId) -> Result<()>;
}
