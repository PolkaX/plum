// Copyright 2020 PolkaX

use cid::Cid;

pub use plum_piece::{PieceInfo, UnpaddedPieceSize};
pub use plum_sector::{PoStProof, SectorId, SectorInfo};
pub use plum_types::{ActorId, Randomness};

pub trait Storage<R: std::io::Read> {
    type Error: std::error::Error;

    fn new_sector(sector: SectorId) -> Result<(), Self::Error>;

    fn add_piece(
        sector: SectorId,
        piece_siezes: UnpaddedPieceSize,
        new_piece_size: UnpaddedPieceSize,
        piece_data: std::io::BufReader<R>,
    ) -> Result<PieceInfo, Self::Error>;
}

pub trait Prover {
    type Error: std::error::Error;

    fn generate_winning_post(
        miner_id: ActorId,
        sector_info: &[SectorInfo],
        randomness: Randomness,
    ) -> Result<PoStProof, Self::Error>;

    fn generate_window_post(
        miner_id: ActorId,
        sector_info: &[SectorInfo],
        randomness: Randomness,
    ) -> Result<PoStProof, Self::Error>;
}

pub type PreCommit1Out = Vec<u8>;
pub type Commit1Out = Vec<u8>;
pub type Proof = Vec<u8>;

pub struct SectorCids {
    pub unsealed: Cid,
    pub sealed: Cid,
}

pub trait Sealer {
    type Error: std::error::Error;

    fn seal_pre_commit1(
        sector: SectorId,
        ticket: Randomness,
        pieces: PieceInfo,
    ) -> Result<PreCommit1Out, Self::Error>;

    fn seal_pre_commit2(sector: SectorId, pc1o: PreCommit1Out) -> Result<SectorCids, Self::Error>;

    fn seal_commit1(
        sector: SectorId,
        ticket: Randomness,
        seed: Randomness,
        pieces: &[PieceInfo],
        cids: SectorCids,
    ) -> Result<Commit1Out, Self::Error>;

    fn seal_commit2(sector: SectorId, c1o: Commit1Out) -> Result<Proof, Self::Error>;

    fn finalize_sector(sector: SectorId) -> Result<(), Self::Error>;
}
