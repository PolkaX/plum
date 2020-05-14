// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// A result type that wraps up the API errors.
pub type Result<T> = std::result::Result<T, ApiError>;

/// The API errors.
pub type ApiError = jsonrpc_client::RpcError;
