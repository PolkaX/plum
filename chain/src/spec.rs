use plum_actor::miner::CHAIN_FINALITYISH;
use plum_types::ChainEpoch;

// TODO: find somewhere more appropriate for whese const settings.

/////////////////////////
// Storage
/////////////////////////
///
pub const UNIXFS_CHUNK_SIZE: u64 = 1 << 20;
///
pub const UNIXFS_LINKS_PER_LEVEL: u64 = 1024;

/////////////////////////
// Consensus / Network
/////////////////////////
/// Seconds
pub const ALLOWABLE_CLOCK_DRIFT: u64 = 1;

/// Epochs
// pub const FORK_LENGTH_THRESHOLD = Finality;

/// Blocks (e)
// pub const BlocksPerEpoch = uint64(builtin.ExpectedLeadersPerEpoch)

/// Epochs
pub const FINALITY: ChainEpoch = CHAIN_FINALITYISH;

// constants for Weight calculation
// The ratio of weight contributed by short-term vs long-term factors in a given round
// pub const WRatioNum = int64(1)
pub const W_RATIO_DEN: u64 = 2;

/////////////////////////
// Proofs
/////////////////////////
/// Epochs
pub const SEAL_RANDOMNESS_LOOKBACK: ChainEpoch = FINALITY;

/// Epochs
pub const SEAL_RANDOMNESS_LOOKBACK_LIMIT: ChainEpoch = SEAL_RANDOMNESS_LOOKBACK + 2000; // TODO: Get from spec specs-actors

/// Maximum lookback that randomness can be sourced from for a seal proof submission
pub const MAX_SEAL_LOOKBACK: ChainEpoch = SEAL_RANDOMNESS_LOOKBACK_LIMIT + 2000; // TODO: Get from specs-actors

/////////////////////////
// Mining
/////////////////////////
/// Epochs
pub const TICKET_RANDOMNESS_LOOKBACK: u64 = 1;
///
pub const WINNING_PO_ST_SECTOR_SET_LOOKBACK: u64 = 10;

/////////////////////////
// Devnet settings
/////////////////////////
///
pub const TOTAL_FILECOIN: u64 = 2_000_000_000;
///
pub const MINING_REWARD_TOTAL: u64 = 1_400_000_000;
///
pub const FILECOIN_PRECISION: u64 = 1_000_000_000_000_000_000;

// Sync
pub const BAD_BLOCK_CACHE_SIZE: u64 = 1 << 15;

/// assuming 4000 messages per round, this lets us not lose any messages across a 10 block reorg.
pub const BLS_SIGNATURE_CACHE_SIZE: u64 = 40000;

/// Size of signature verification cache
/// 32k keeps the cache around 10MB in size, max
pub const VERIF_SIG_CACHE_SIZE: u64 = 32000;

/////////////////////////
// Limits
/////////////////////////
/// TODO: If this is gonna stay, it should move to specs-actors
pub const BLOCK_MESSAGE_LIMIT: u64 = 512;
///
pub const BLOCK_GAS_LIMIT: u64 = 100_000_000;

///
pub const DRAND_COEFFS: [&str; 4] = [
    "82c279cce744450e68de98ee08f9698a01dd38f8e3be3c53f2b840fb9d09ad62a0b6b87981e179e1b14bc9a2d284c985",
    "82d51308ad346c686f81b8094551597d7b963295cbf313401a93df9baf52d5ae98a87745bee70839a4d6e65c342bd15b",
    "94eebfd53f4ba6a3b8304236400a12e73885e5a781509a5c8d41d2e8b476923d8ea6052649b3c17282f596217f96c5de",
    "8dc4231e42b4edf39e86ef1579401692480647918275da767d3e558c520d6375ad953530610fd27daf110187877a65d0",
];
