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
    #[structopt(short = "p", long = "peer")]
    peer: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Plum {
    pub fn parse_and_prepare(&self) {
        println!("args: {:?}", self);
    }
}

pub fn run() {
    let opt = Plum::from_args();
    // opt.unset_setting(AppSettings::SubcommandRequiredElseHelp);

    opt.parse_and_prepare();

    env_logger::init();
    let peer_ip = opt.peer;
    let (exit_send, exit) = exit_future::signal();
    let mut runtime = Runtime::new().expect("failed to start runtime on current thread");
    let task_executor = runtime.executor();
    let network_state = lp2p::NetworkState::default();
    lp2p::initialize(task_executor, network_state, peer_ip);
    let _ = runtime.block_on(exit);
    exit_send.fire();
}
