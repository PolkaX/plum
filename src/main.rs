// Copyright 2019 杭州链网科技 Team.
use tokio::runtime::Runtime;

fn main() {
    env_logger::init();
    let peer_ip = cli::get_terminal();
    let (exit_send, exit) = exit_future::signal();
    let mut runtime = Runtime::new().expect("failed to start runtime on current thread");
    let task_executor = runtime.executor();
    let network_state = lp2p::NetworkState::default();
    lp2p::initialize(task_executor, network_state, peer_ip);
    let _ = runtime.block_on(exit);
    exit_send.fire();
}
