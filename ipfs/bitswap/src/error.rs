// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

#[derive(Debug, thiserror::Error)]
pub enum BitswapError {
    #[error("Protobuf decode error: {0}")]
    ProtobufDecodeError(#[from] prost::DecodeError),
}
