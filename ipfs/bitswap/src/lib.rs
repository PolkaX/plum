// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The implementation of the IPFS bitswap protocol.
//!
//! See the [bitswap docs](https://docs.ipfs.io/concepts/bitswap/) for details.
//!
//! # Bitswap
//!
//! Bitswap is a core module of IPFS for exchanging blocks of data.
//! It directs the requesting and sending of blocks to and from other peers in the network.
//! Bitswap is a message-based protocol where all messages contain want-lists or blocks.
//! Bitswap has a [Go implementation](https://github.com/ipfs/go-bitswap) and a [JavaScript implementation](https://github.com/ipfs/js-ipfs-bitswap).
//!
//! Bitswap has two main jobs:
//!
//! - Acquire blocks requested by the client from the network.
//! - Send blocks in its possession to other peers who want them.
//!
//! ## How Bitswap works
//!
//! IPFS breaks up files into chunks of data called blocks.
//! These blocks are identified by a content identifier (CID).
//! When nodes running the Bitswap protocol want to fetch a file, they send out want-lists to other peers.
//! A want-list is a list of CIDs for blocks a peer wants to receive.
//! Each node remembers which blocks its peers want.
//! Each time a node receives a block, it checks if any of its peers want the block, and sends it to them if they do.
//!
//! Here is a simplified version of a `want-list`:
//!
//! ```txt
//! Want-list {
//!   QmZtmD2qt6fJot32nabSP3CUjicnypEBz7bHVDhPQt9aAy, WANT,
//!   QmTudJSaoKxtbEnTddJ9vh8hbN84ZLVvD5pNpUaSbxwGoa, WANT,
//!   ...
//! }
//! ```
//!
//! ### Discovery
//!
//! To find peers that have a file, a node running the Bitswap protocol first sends a request
//! called a want-have to all the peers it is connected to.
//! This want-have request contains the CID of the root block of the file
//! (the root block is at the top of the DAG of blocks that make up the file).
//! Peers that have the root block send a have response, and are added to a session.
//! Peers that don't have the block send a dont-have response.
//! If none of the peers have the root block, Bitswap queries the Distributed Hash Table (DHT)
//! to ask who can provide the root block.
//!
//! ![discovery process](https://docs.ipfs.io/assets/img/diagram-of-the-want-have-want-block-process.6ef862a2.png)
//!
//! ### Transfer
//!
//! Once peers have been added to a session, for each block that the client wants,
//! Bitswap sends want-have to each session peer to find out which peers have the block.
//! Peers respond with have or dont_have.
//! Bitswap builds up a map of which nodes have and don't have each block.
//! Bitswap sends want-block to peers that have the block and they respond with the block itself.
//! If no peers have the block Bitswap queries the DHT to find providers who have the block.
//!

#![deny(missing_docs)]

// mod behaviour;
mod error;
// mod ledger;
// mod message;
mod prefix;
mod proto;
// mod protocol;
