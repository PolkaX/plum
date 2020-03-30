// Core network constants

// /////
// Storage

pub const UNIXFS_CHUNK_SIZE: u64 = 1 << 20;
pub const UNIXFS_LINKS_PER_LEVEL: u64 = 1024;


pub const SECTOR_CHALLENGE_RATIO_DIV: u64 = 25;

// /////
// Payments

// Epochs
pub const PAYMENT_CHANNEL_CLOSING_DELAY: u64 = 6 * 60 * 60 / BlockDelay; // six hours

// /////
// Consensus / Network

// Seconds
pub const ALLOWABLE_CLOCK_DRIFT: u64 = 1;

// Epochs
pub const FORK_LENGTH_THRESHOLD: u64 = FINALITY;

// Blocks (e)
pub const BLOCKS_PER_EPOCH: u64 = 5;

// Epochs
pub const FINALITY: u64 = 500;

// constants for Weight calculation
// The ratio of weight contributed by short-term vs long-term factors in a given round
pub const WRATIO_NUM: u64 = 1;
pub const WRATIO_DEN: u64 = 2;

// /////
// Proofs

// Epochs
pub const SEAL_RANDOMNESS_LOOKBACK: u64 = FINALITY;

// Epochs
pub const SEAL_RANDOMNESS_LOOKBACK_LIMIT: u64 = SEAL_RANDOMNESS_LOOKBACK + 2000;

// Maximum lookback that randomness can be sourced from for a seal proof submission
pub const MAX_SEAL_LOOKBACK: u64 = SEAL_RANDOMNESS_LOOKBACK_LIMIT + 2000;

// /////
// Mining

// Epochs
pub const EC_RANDOMNESS_LOOKBACK: u64 = 300;

pub const POWER_COLLATERAL_PROPORTION: u64 = 5;
pub const PER_CAPITA_COLLATERAL_PROPORTION: u64 = 1;
pub const COLLATERAL_PRECISION: u64 = 1000;

// /////
// Devnet settings

pub const TOTAL_FILECOIN: u64 = 2_000_000_000;
pub const MINING_REWARD_TOTAL: u64 = 1_400_000_000;

pub const INITIAL_REWARD_STR: &'static str = "153856861913558700202";

pub const FILECOIN_PRECISION: u64 = 1_000_000_000_000_000_000;

// Sync
pub const BAD_BLOCK_CACHE_SIZE: u64 = 1 << 15;

// assuming 4000 messages per round, this lets us not lose any messages across a
// 10 block reorg.
pub const BLS_SIGNATURE_CACHE_SIZE: u64 = 40000;

// ///////
// Limits

pub const BLOCK_MESSAGE_LIMIT: u64 = 512;
pub const MINER_MAX_SECTORS: u64 = 1 << 48;
