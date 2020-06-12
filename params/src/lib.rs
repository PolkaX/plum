// Core network constants

use std::sync::Once;

use plum_sector::SectorSize;
use plum_types::ChainEpoch;

static mut PARAMS: Params = Params {
    unixfs_chunk_size: 0,
    unixfs_links_per_level: 0,
    sector_challenge_ratio_div: 0,
    payment_channel_closing_delay: 0,
    allowable_clock_drift: 0,
    fork_length_threshold: 0,
    blocks_per_epoch: 0,
    finality: 0,
    wratio_num: 0,
    wratio_den: 0,
    seal_randomness_lookback: 0,
    seal_randomness_lookback_limit: 0,
    max_seal_lookback: 0,
    ec_randomness_lookback: 0,
    power_collateral_proportion: 0,
    per_capita_collateral_proportion: 0,
    collateral_precision: 0,
    bad_block_cache_size: 0,
    bls_signature_cache_size: 0,
    block_message_limit: 0,
    miner_max_sectors: 0,
    chain: Chain {
        sector_sizes: [0; 8],
        block_delay: 0,
        propagation_delay: 0,
        fallback_po_st_delay: 0,
        slashable_power_delay: 0,
        interactive_po_rep_delay: 0,
        interactive_po_rep_confidence: 0,
        minimum_miner_power: 0,
    },
    fil: Fil {
        total_filecoin: 0,
        mining_reward_total: 0,
        initial_reward: 0,
        filecoin_precision: 0,
    },
};
static INIT: Once = Once::new();

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Network {
    Mainnet,
    Testnet,
    Dev,
}

pub fn params() -> &'static Params {
    unsafe {
        // if not do init_params, use testnet to init
        init_params(Network::Testnet);
        &PARAMS
    }
}

pub fn init_params(networt: Network) {
    unsafe {
        INIT.call_once(|| {
            PARAMS = Params::init(networt);
        });
    }
}

pub struct Params {
    // Storage
    pub unixfs_chunk_size: u64,
    pub unixfs_links_per_level: u64,
    pub sector_challenge_ratio_div: u64,
    // Payments
    // Epochs
    pub payment_channel_closing_delay: ChainEpoch,
    // Consensus / Network
    // Seconds
    pub allowable_clock_drift: ChainEpoch,
    // Epochs
    pub fork_length_threshold: ChainEpoch,
    // Blocks (e)
    pub blocks_per_epoch: u64,
    // Epochs
    pub finality: ChainEpoch,
    // constants for Weight calculation
    // The ratio of weight contributed by short-term vs long-term factors in a given round
    pub wratio_num: u64,
    pub wratio_den: u64,
    // proofs
    // Epochs
    pub seal_randomness_lookback: ChainEpoch,
    // Epochs
    pub seal_randomness_lookback_limit: ChainEpoch,
    // Maximum lookback that randomness can be sourced from for a seal proof submission
    pub max_seal_lookback: ChainEpoch,
    // Epochs
    pub ec_randomness_lookback: ChainEpoch,
    pub power_collateral_proportion: u64,
    pub per_capita_collateral_proportion: u64,
    pub collateral_precision: u64,
    // Sync
    pub bad_block_cache_size: u64,
    // assuming 4000 messages per round, this lets us not lose any messages across a
    // 10 block reorg.
    pub bls_signature_cache_size: u64,
    // Limits
    pub block_message_limit: u64,
    pub miner_max_sectors: u64,
    pub chain: Chain,
    pub fil: Fil,
}

pub struct Fil {
    pub total_filecoin: u64,
    pub mining_reward_total: u64,
    pub initial_reward: u128,
    pub filecoin_precision: u64,
}

pub struct Chain {
    pub sector_sizes: [SectorSize; 8],
    pub block_delay: ChainEpoch,
    pub propagation_delay: ChainEpoch,
    // fallback_po_st_delay is the number of epochs the miner needs to wait after
    //  ElectionPeriodStart before starting fallback post computation
    //
    // Epochs
    pub fallback_po_st_delay: ChainEpoch,
    // slashable_power_delay is the number of epochs after ElectionPeriodStart, after
    // which the miner is slashed
    //
    // Epochs
    pub slashable_power_delay: ChainEpoch,
    // Epochs
    pub interactive_po_rep_delay: ChainEpoch,
    // Epochs
    pub interactive_po_rep_confidence: ChainEpoch,
    // Bytes
    pub minimum_miner_power: u64,
}

fn testnet_chain() -> Chain {
    Chain {
        sector_sizes: [32 << 30, 0, 0, 0, 0, 0, 0, 0],
        block_delay: 45,
        propagation_delay: 6,
        fallback_po_st_delay: 30,
        slashable_power_delay: 200,
        interactive_po_rep_delay: 8,
        interactive_po_rep_confidence: 6,
        minimum_miner_power: 512 << 30, // 512GB
    }
}

fn testnet_fil() -> Fil {
    dev_fil()
}

fn dev_chain() -> Chain {
    Chain {
        sector_sizes: [1024, 0, 0, 0, 0, 0, 0, 0],
        block_delay: 6,
        propagation_delay: 3,
        fallback_po_st_delay: 10,
        slashable_power_delay: 20,
        interactive_po_rep_delay: 2,
        interactive_po_rep_confidence: 6,
        minimum_miner_power: 2 << 10, // 2KiB
    }
}

fn dev_fil() -> Fil {
    Fil {
        total_filecoin: 2_000_000_000,
        mining_reward_total: 1_400_000_000,
        initial_reward: 153856861913558700202,
        filecoin_precision: 1_000_000_000_000_000_000,
    }
}

impl Params {
    pub fn init(network: Network) -> Params {
        let (chain, fil) = match network {
            Network::Mainnet => unimplemented!("not impl yet"),
            Network::Testnet => (testnet_chain(), testnet_fil()),
            Network::Dev => (dev_chain(), dev_fil()),
        };
        let finality = 500;
        let seal_randomness_lookback = finality;
        let seal_randomness_lookback_limit = seal_randomness_lookback + 2000;
        Params {
            unixfs_chunk_size: 1 << 20,
            unixfs_links_per_level: 1024,
            sector_challenge_ratio_div: 25,
            payment_channel_closing_delay: 6 * 60 * 60 / chain.block_delay,
            allowable_clock_drift: 1,
            fork_length_threshold: finality,
            blocks_per_epoch: 5,
            finality,
            wratio_num: 1,
            wratio_den: 2,
            seal_randomness_lookback,
            seal_randomness_lookback_limit,
            max_seal_lookback: seal_randomness_lookback_limit + 2000,
            ec_randomness_lookback: 300,
            power_collateral_proportion: 5,
            per_capita_collateral_proportion: 1,
            collateral_precision: 1000,
            bad_block_cache_size: 1 << 15,
            bls_signature_cache_size: 40000,
            block_message_limit: 512,
            miner_max_sectors: 1 << 48,
            chain,
            fil,
        }
    }
}
