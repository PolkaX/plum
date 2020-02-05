// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! Available RPC methods types and ids.

use serde::{Deserialize, Serialize};

/* Request/Response data structures for RPC methods */

/* Requests */

pub type RequestId = usize;

/// The STATUS request/response handshake message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusMessage {
    /// The fork version of the chain we are broadcasting.
    pub fork_version: [u8; 4],
}

/// The reason given for a `Goodbye` message.
///
/// Note: any unknown `u64::into(n)` will resolve to `Goodbye::Unknown` for any unknown `n`,
/// however `GoodbyeReason::Unknown.into()` will go into `0_u64`. Therefore de-serializing then
/// re-serializing may not return the same bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GoodbyeReason {
    /// This node has shutdown.
    ClientShutdown = 1,

    /// Incompatible networks.
    IrrelevantNetwork = 2,

    /// Error/fault in the RPC.
    Fault = 3,

    /// Unknown reason.
    Unknown = 0,
}

impl From<u64> for GoodbyeReason {
    fn from(id: u64) -> GoodbyeReason {
        match id {
            1 => GoodbyeReason::ClientShutdown,
            2 => GoodbyeReason::IrrelevantNetwork,
            3 => GoodbyeReason::Fault,
            _ => GoodbyeReason::Unknown,
        }
    }
}

impl Into<u64> for GoodbyeReason {
    fn into(self) -> u64 {
        self as u64
    }
}

// https://github.com/filecoin-project/lotus/blob/e7a1be4dde/chain/blocksync/blocksync.go#L34
/// Request a number of fil block from a peer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockSyncRequest {
    pub start: Vec<cid::Cid>,
    pub length: u64,
    pub options: u64,
}

/* RPC Handling and Grouping */
// Collection of enums and structs used by the Codecs to encode/decode RPC messages

// https://github.com/filecoin-project/lotus/blob/e7a1be4dde/chain/blocksync/blocksync.go#L67
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RPCResponse {
    /// A HELLO message.
    Status(StatusMessage),

    /// A response to a get BLOCK_SYNC_REQUEST request. A None response signifies the end of the
    /// batch.
    BlockSyncRequest(Vec<u8>),
}

/// Indicates which response is being terminated by a stream termination response.
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseTermination {
    /// Block sync request stream termination.
    BlockSyncRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RPCErrorResponse {
    /// The response is a successful.
    Success(RPCResponse),

    /// The response was invalid.
    InvalidRequest(ErrorMessage),

    /// The response indicates a server error.
    ServerError(ErrorMessage),

    /// There was an unknown response.
    Unknown(ErrorMessage),

    /// Received a stream termination indicating which response is being terminated.
    StreamTermination(ResponseTermination),
}

impl RPCErrorResponse {
    /// Used to encode the response in the codec.
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            RPCErrorResponse::Success(_) => Some(0),
            RPCErrorResponse::InvalidRequest(_) => Some(1),
            RPCErrorResponse::ServerError(_) => Some(2),
            RPCErrorResponse::Unknown(_) => Some(255),
            RPCErrorResponse::StreamTermination(_) => None,
        }
    }

    /// Tells the codec whether to decode as an RPCResponse or an error.
    pub fn is_response(response_code: u8) -> bool {
        match response_code {
            0 => true,
            _ => false,
        }
    }

    /// Builds an RPCErrorResponse from a response code and an ErrorMessage
    pub fn from_error(response_code: u8, err: ErrorMessage) -> Self {
        match response_code {
            1 => RPCErrorResponse::InvalidRequest(err),
            2 => RPCErrorResponse::ServerError(err),
            _ => RPCErrorResponse::Unknown(err),
        }
    }

    /// Specifies which response allows for multiple chunks for the stream handler.
    pub fn multiple_responses(&self) -> bool {
        match self {
            RPCErrorResponse::Success(resp) => match resp {
                RPCResponse::Status(_) => false,
                RPCResponse::BlockSyncRequest(_) => true,
            },
            RPCErrorResponse::InvalidRequest(_) => true,
            RPCErrorResponse::ServerError(_) => true,
            RPCErrorResponse::Unknown(_) => true,
            // Stream terminations are part of responses that have chunks
            RPCErrorResponse::StreamTermination(_) => true,
        }
    }

    /// Returns true if this response is an error. Used to terminate the stream after an error is
    /// sent.
    pub fn is_error(&self) -> bool {
        match self {
            RPCErrorResponse::Success(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    /// The UTF-8 encoded Error message string.
    pub error_message: Vec<u8>,
}

impl ErrorMessage {
    pub fn as_string(&self) -> String {
        String::from_utf8(self.error_message.clone()).unwrap_or_else(|_| "".into())
    }
}

impl std::fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status Message: Fork Version: {:?}, Finalized Root: , Finalized Epoch: , Head Root: Head Slot: ", self.fork_version)
    }
}

impl std::fmt::Display for RPCResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RPCResponse::Status(status) => write!(f, "{}", status),
            RPCResponse::BlockSyncRequest(_) => write!(f, "<BlockSyncRequest>"),
        }
    }
}

impl std::fmt::Display for RPCErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RPCErrorResponse::Success(res) => write!(f, "{}", res),
            RPCErrorResponse::InvalidRequest(err) => write!(f, "Invalid Request: {:?}", err),
            RPCErrorResponse::ServerError(err) => write!(f, "Server Error: {:?}", err),
            RPCErrorResponse::Unknown(err) => write!(f, "Unknown Error: {:?}", err),
            RPCErrorResponse::StreamTermination(_) => write!(f, "Stream Termination"),
        }
    }
}

impl std::fmt::Display for GoodbyeReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoodbyeReason::ClientShutdown => write!(f, "Client Shutdown"),
            GoodbyeReason::IrrelevantNetwork => write!(f, "Irrelevant Network"),
            GoodbyeReason::Fault => write!(f, "Fault"),
            GoodbyeReason::Unknown => write!(f, "Unknown Reason"),
        }
    }
}

impl std::fmt::Display for BlockSyncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Start: {:?}, Length: {}, Options: {}",
            self.start, self.length, self.options
        )
    }
}
