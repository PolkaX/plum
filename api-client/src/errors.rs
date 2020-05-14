// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

///
pub type Result<T> = std::result::Result<T, ApiError>;

///
pub type ApiError = jsonrpc_client::RpcError;
