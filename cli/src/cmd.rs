// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

use structopt::clap::arg_enum;
use structopt::StructOpt;

use crate::run_lp2p;
use wallet::crypto;

#[derive(StructOpt, Debug, Clone)]
pub enum Auth {
    /// Create token
    #[structopt(name = "create-token")]
    CreateToken,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Chain {
    /// Print chain head
    #[structopt(name = "head")]
    Head,
    #[structopt(name = "get-block")]
    GetBlock,
    /// Get and print a message by its cid
    #[structopt(name = "get-message")]
    GetMessage,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Client {
    /// Import data
    #[structopt(name = "import")]
    Import,
    /// List locally imported data
    #[structopt(name = "local")]
    Local,
    /// Initialize storage deal with a miner
    #[structopt(name = "deal")]
    Deal,
    /// Find data in the network
    #[structopt(name = "find")]
    Find,
    /// Retrive data from network
    #[structopt(name = "retrieve")]
    Retrive,
    /// Find a miner to ask
    #[structopt(name = "query-ask")]
    QueryAsk,
    /// List storage market deals
    #[structopt(name = "list-deals")]
    ListDeals,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Miner {
    /// Create a new storage market actor
    #[structopt(name = "create")]
    Create,
    /// Manually unregister miner actor
    #[structopt(name = "unregister")]
    Unregister,
}

#[derive(StructOpt, Debug, Clone)]
pub enum MessagePool {
    #[structopt(name = "pending")]
    /// Get pending messages
    Pending,
    #[structopt(name = "subscribe")]
    /// Subscribe to message pool changes
    Subscribe,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Network {
    /// Print peers
    #[structopt(name = "peers")]
    Peers,
    /// List listen addresses
    #[structopt(name = "listen")]
    Listen,
    /// Connect to a peer
    #[structopt(name = "connect")]
    Connect {
        /// Specify an IPFS peer ip to connect
        #[structopt(short = "p", long = "peer")]
        peer: String,
    },
    /// Get node identity
    #[structopt(name = "id")]
    Id,
}

impl Network {
    pub fn execute(&self) {
        match self {
            Network::Connect { peer } => run_lp2p(Some(peer.to_owned())),
            _ => unimplemented!(),
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
pub enum PaymentChannel {
    /// Create a new payment channel or get existing one
    #[structopt(name = "get")]
    Get,
    /// List all locally registered payment channels
    #[structopt(name = "list")]
    List,
    /// Interact with payment channel vouchers
    #[structopt(name = "voucher")]
    Voucher(Voucher),
}

#[derive(StructOpt, Debug, Clone)]
pub enum Voucher {
    /// Create a signed payment channel voucher
    #[structopt(name = "create")]
    Create,
    /// Check validity of payment channel voucher
    #[structopt(name = "check")]
    Check,
    /// Add payment channel voucher to local datastore
    #[structopt(name = "add")]
    Add,
    /// List stored vouchers for a given payment channel
    #[structopt(name = "list")]
    List,
    /// Print voucher with highest value that is currently spendable
    #[structopt(name = "best-spendable")]
    BestSpendable,
    /// Submit voucher to chain to update payment channel state
    #[structopt(name = "submit")]
    Submit,
}

#[derive(StructOpt, Debug, Clone)]
pub enum State {
    /// Query network or miner power
    #[structopt(name = "power")]
    Power,
    /// Query the proving set of a miner
    #[structopt(name = "proving")]
    Proving,
    /// Replay a particular message within a tipset
    #[structopt(name = "replay")]
    Replay,
    /// Get minimum miner pledge collateral
    #[structopt(name = "pledge-collateral")]
    PledgeCollateral,
    /// List all miners in the network
    #[structopt(name = "list-miners")]
    ListMiners,
    /// List all actors in the network
    #[structopt(name = "list-actors")]
    ListActors,
    /// Print actor information
    #[structopt(name = "get-actor")]
    GetActor,
    /// Find coresponding ID address
    #[structopt(name = "lookup")]
    Lookup,
}

#[derive(StructOpt, Debug, Clone)]
pub enum Sync {
    /// Check sync status
    #[structopt(name = "status")]
    Status,
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum KeyType {
        Bls,
        Secp256k1,
    }
}

#[derive(StructOpt, Debug, Clone)]
pub enum Wallet {
    #[structopt(name = "new")]
    /// Generate a new key of the given type
    New {
        #[structopt(short="t", long="type", possible_values = &KeyType::variants(), case_insensitive = true)]
        key_type: KeyType,
    },
    #[structopt(name = "list")]
    /// List all the key in keystore
    List {
        #[structopt(short = "p", long = "keystore_path", case_insensitive = true)]
        keystore_path: Option<String>,
    },
}

impl Wallet {
    pub fn execute(&self) {
        match self {
            Wallet::New { key_type } => {
                let keytype = match key_type {
                    KeyType::Bls => crypto::key_types::BLS,
                    KeyType::Secp256k1 => crypto::key_types::SECP256K1,
                };
                wallet::Wallet::new_address(keytype.to_owned())
            }
            Wallet::List { keystore_path } => wallet::Wallet::wallet_list(keystore_path.to_owned()),
            _ => unimplemented!(),
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
pub enum Command {
    /// Manage RPC permissions
    #[structopt(name = "auth")]
    Auth(Auth),
    /// Interact with filecoin blockchain
    #[structopt(name = "chain")]
    Chain(Chain),
    /// Make deals, store data, retrieve data
    #[structopt(name = "client")]
    Client(Client),
    /// Manage miner actor
    #[structopt(name = "miner")]
    Miner(Miner),
    /// Manage message pool
    #[structopt(name = "mpool")]
    MessagePool(MessagePool),
    /// Manage P2P network
    #[structopt(name = "network")]
    Network(Network),
    /// Inspect or interact with the chain syncer
    #[structopt(name = "sync")]
    Sync(Sync),
    /// Manage payment channels
    #[structopt(name = "paych")]
    PaymentChannel(PaymentChannel),
    /// Interact with and query filecoin chain state
    #[structopt(name = "state")]
    State(State),
    /// Manage wallet
    #[structopt(name = "wallet")]
    Wallet(Wallet),
    /// Send funds between accounts
    #[structopt(name = "transfer")]
    Transfer {
        /// Receiver
        #[structopt(name = "target")]
        target: String,
        /// Value
        #[structopt(name = "value")]
        value: u64,
    },
    /// Fetch proving parameters
    #[structopt(name = "fetch-param")]
    FetchParam {
        /// Only download the verified keys
        #[structopt(name = "only-verify-keys")]
        only_verify_keys: bool,
    },
}
