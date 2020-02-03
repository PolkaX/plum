pub mod behaviour;
pub mod config;
pub mod service;

pub use config::Libp2pConfig;
pub use libp2p::gossipsub::{MessageId, TopicHash};
pub use libp2p::{Multiaddr, PeerId};
