// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod blocksync;
mod hello;

pub use self::blocksync::{
    BlockSyncCodec, BlockSyncProtocolName, BlockSyncRequest, BlockSyncResponse, BlockSyncTipset,
    BLOCKSYNC_PROTOCOL_ID,
};
pub use self::hello::{
    HelloCodec, HelloProtocolName, HelloRequest, HelloResponse, HELLO_PROTOCOL_ID,
};

fn other_io_error(err: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}
