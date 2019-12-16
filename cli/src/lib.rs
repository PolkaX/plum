// Copyright 2019 PolkaX. Licensed under GPL-3.0.

pub mod cmd;

use structopt::clap::AppSettings;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use self::cmd::Command;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "plum")]
#[structopt(setting = AppSettings::ArgRequiredElseHelp)]
pub struct Plum {
    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Plum {
    pub fn execute(&self) {
        match &self.cmd {
            Command::Network(network) => network.execute(),
            _ => unimplemented!(),
        }
    }
}

pub fn run_lp2p(peer_ip: Option<String>) {
    env_logger::init();
    let (exit_send, exit) = exit_future::signal();
    let mut runtime = Runtime::new().expect("failed to start runtime on current thread");
    let task_executor = runtime.executor();
    let network_state = lp2p::NetworkState::default();
    lp2p::initialize(task_executor, network_state, peer_ip);
    let _ = runtime.block_on(exit);
    exit_send.fire();
}

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        run_lp2p(None);
    } else {
        let plum = Plum::from_iter(args.iter());
        plum.execute();
    }
}
