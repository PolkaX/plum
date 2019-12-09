// Copyright 2019 PolkaX

pub mod cmd;

use structopt::clap::AppSettings;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use cmd::Command;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "plum")]
#[structopt(setting = AppSettings::ArgRequiredElseHelp)]
pub struct Plum {
    /// Specify an IPFS peer ip to connect
    #[structopt(short = "p", long = "peer")]
    peer: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();
    let peer_ip = if args.len() == 1 {
        None
    } else {
        let opt = Plum::from_iter(args.iter());
        opt.peer
    };

    env_logger::init();
    let (exit_send, exit) = exit_future::signal();
    let mut runtime = Runtime::new().expect("failed to start runtime on current thread");
    let task_executor = runtime.executor();
    let network_state = lp2p::NetworkState::default();
    lp2p::initialize(task_executor, network_state, peer_ip);
    let _ = runtime.block_on(exit);
    exit_send.fire();
}
