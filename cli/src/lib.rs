// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub mod cmd;

use std::io::Write;

use ansi_term::Colour;
use lazy_static::lazy_static;
use log::info;
use plum_libp2p::Multiaddr;
use regex::Regex;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use crate::cmd::Command;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "plum")]
#[structopt(setting = AppSettings::ArgRequiredElseHelp)]
pub struct Plum {
    /// Set a custom logging filter.
    #[structopt(short = "l", long = "log", value_name = "LOG_PATTERN")]
    pub log: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Plum {
    pub fn execute(&self) {
        match &self.cmd {
            Command::Network(network) => network.execute(),
            /*Command::Wallet(wallet) => wallet.execute(),*/
            _ => unimplemented!(),
        }
    }
}

pub struct Client;

impl chain::Client for Client {
    fn info(&self) -> chain::Info {
        chain::Info {
            heaviest_tip_set: 101u8,
            heaviest_tip_set_weight: 1000u128,
            genesis_hash: 0u8,
            best_hash: 88u8,
        }
    }
}

pub fn run_lp2p(peer_ip: Option<Multiaddr>) {
    let (exit_send, exit) = exit_future::signal();
    let mut runtime = Runtime::new().expect("failed to start runtime on current thread");
    let task_executor = runtime.executor();

    // TODO: mock client and intergate with network service.
    let _client = Client;

    let mut network_config = plum_libp2p::Libp2pConfig::default();
    if let Some(peer) = peer_ip {
        network_config.bootnodes.push(peer);
    }

    let _network_service = network::service::Service::spawn(&network_config, &task_executor);

    let _ = runtime.block_on(exit);
    exit_send.fire();
}

fn kill_color(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\x1b\\[[^m]+m").expect("Error initializing color regex");
    }
    RE.replace_all(s, "").to_string()
}

fn init_logger(custom_log: Option<String>) {
    let mut builder = env_logger::Builder::new();
    // Disable info logging by default for some modules:
    builder.filter(Some("ws"), log::LevelFilter::Off);
    builder.filter(Some("hyper"), log::LevelFilter::Warn);
    // Enable info for others.
    builder.filter(None, log::LevelFilter::Info);

    if let Ok(lvl) = std::env::var("RUST_LOG") {
        builder.parse_filters(&lvl);
    }

    let pattern = custom_log.as_ref().map(|v| v.as_ref()).unwrap_or("");
    builder.parse_filters(pattern);
    let isatty = atty::is(atty::Stream::Stderr);
    let enable_color = isatty;

    builder.format(move |buf, record| {
        let now = time::now();
        let timestamp =
            time::strftime("%Y-%m-%d %H:%M:%S", &now).expect("Error formatting log timestamp");

        let mut output = if custom_log.is_none() {
            format!(
                "{} {}",
                Colour::RGB(96, 96, 96).paint(timestamp),
                record.args()
            )
        } else if log::max_level() <= log::LevelFilter::Info {
            format!(
                "{} {} {} {}",
                Colour::RGB(96, 96, 96).paint(timestamp),
                Colour::Yellow.paint(format!("{}", record.level())),
                record.target(),
                record.args()
            )
        } else {
            let name = ::std::thread::current()
                .name()
                .map_or_else(Default::default, |x| {
                    format!("{}", Colour::Blue.bold().paint(x))
                });
            let millis = (now.tm_nsec as f32 / 1_000_000.0).round() as usize;
            let timestamp = format!("{}.{:03}", timestamp, millis);
            format!(
                "{} {} {} {}  {}",
                Colour::RGB(96, 96, 96).paint(timestamp),
                name,
                Colour::Yellow.paint(format!("{}", record.level())),
                record.target(),
                record.args()
            )
        };

        if !isatty && record.level() <= log::Level::Info && atty::is(atty::Stream::Stdout) {
            // duplicate INFO/WARN output to console
            println!("{}", output);
        }

        if !enable_color {
            output = kill_color(output.as_ref());
        }

        writeln!(buf, "{}", output)
    });

    if builder.try_init().is_err() {
        info!("Not registering Plum logger, as there is already a global logger registered!");
    }
}

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        init_logger(None);
        run_lp2p(None);
    } else {
        let plum = Plum::from_iter(args.iter());
        init_logger(plum.log.clone());
        plum.execute();
    }
}
