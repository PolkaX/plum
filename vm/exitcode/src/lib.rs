// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The exit code from the VM execution.

#![deny(missing_docs)]

use std::convert::TryFrom;
use std::error;
use std::fmt;

use minicbor::{decode, encode, Decoder, Encoder};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The exit code from the VM execution
#[derive(
    Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(i64)]
pub enum ExitCode {
    // ========================================================================
    // The system error codes are reserved for use by the runtime.
    // No actor may use one explicitly.
    // Correspondingly, no runtime invocation should abort with an exit code outside this list.
    // We could move these definitions out of this package and into the runtime spec.
    /// Exit the VM execution successfully.
    Ok = 0,
    /// Indicates that the actor identified as the sender of a message is not valid as a message sender:
    /// - not present in the state tree
    /// - not an account actor (for top-level messages)
    /// - code CID is not found or invalid
    /// (not found in the state tree, not an account, has no code).
    SysErrSenderInvalid = 1,
    /// Indicates that the sender of a message is not in a state to send the message:
    /// - invocation out of sequence (mismatched CallSeqNum)
    /// - insufficient funds to cover execution
    SysErrSenderStateInvalid = 2,
    /// Indicates failure to find a method in an actor.
    SysErrInvalidMethod = 3,
    /// Indicates non-decodable or syntactically invalid parameters for a method.
    SysErrInvalidParameters = 4,
    /// Indicates that the receiver of a message is not valid (and cannot be implicitly created).
    SysErrInvalidReceiver = 5,
    /// Indicates that a message sender has insufficient balance for the value being sent.
    /// Note that this is distinct from SysErrSenderStateInvalid when a top-level sender can't cover
    /// value transfer + gas. This code is only expected to come from inter-actor sends.
    SysErrInsufficientFunds = 6,
    /// Indicates message execution (including subcalls) used more gas than the specified limit.
    SysErrOutOfGas = 7,
    /// Indicates message execution is forbidden for the caller by runtime caller validation.
    SysErrForbidden = 8,
    /// Indicates actor code performed a disallowed operation. Disallowed operations include:
    /// - mutating state outside of a state acquisition block
    /// - failing to invoke caller validation
    /// - aborting with a reserved exit code (including success or a system error).
    SysErrorIllegalActor = 9,
    /// Indicates an invalid argument passed to a runtime method.
    SysErrorIllegalArgument = 10,
    /// Indicates  an object failed to de/serialize for storage.
    SysErrSerialization = 11,

    /// Reserved error.
    SysErrorReserved3 = 12,
    /// Reserved error.
    SysErrorReserved4 = 13,
    /// Reserved error.
    SysErrorReserved5 = 14,
    /// Reserved error.
    SysErrorReserved6 = 15,

    // ========================================================================
    // Common error codes that may be shared by different actors.
    // Actors may also define their own codes, including redefining these values.

    // The initial range of exit codes is reserved for system errors.
    // Actors may define codes starting with this one.
    // FirstActorErrorCode = 16,
    /// Indicates a method parameter is invalid.
    ErrIllegalArgument = 16,
    /// Indicates a requested resource does not exist.
    ErrNotFound = 17,
    /// Indicates an action is disallowed.
    ErrForbidden = 18,
    /// Indicates a balance of funds is insufficient.
    ErrInsufficientFunds = 19,
    /// Indicates an actor's internal state is invalid.
    ErrIllegalState = 20,
    /// Indicates de/serialization failure within actor code.
    ErrSerialization = 21,

    // Common error codes stop here.
    // If you define a common error code above this value it will have conflicting interpretations
    // FirstActorSpecificExitCode = 32,

    // ========================================================================
    /// An error code intended to be replaced by different code structure or a more descriptive error.
    ErrPlaceholder = 1000,
}

impl From<ExitCode> for i64 {
    fn from(code: ExitCode) -> Self {
        match code {
            ExitCode::Ok => 0,
            ExitCode::SysErrSenderInvalid => 1,
            ExitCode::SysErrSenderStateInvalid => 2,
            ExitCode::SysErrInvalidMethod => 3,
            ExitCode::SysErrInvalidParameters => 4,
            ExitCode::SysErrInvalidReceiver => 5,
            ExitCode::SysErrInsufficientFunds => 6,
            ExitCode::SysErrOutOfGas => 7,
            ExitCode::SysErrForbidden => 8,
            ExitCode::SysErrorIllegalActor => 9,
            ExitCode::SysErrorIllegalArgument => 10,
            ExitCode::SysErrSerialization => 11,
            ExitCode::SysErrorReserved3 => 12,
            ExitCode::SysErrorReserved4 => 13,
            ExitCode::SysErrorReserved5 => 14,
            ExitCode::SysErrorReserved6 => 15,
            // ================================================================
            ExitCode::ErrIllegalArgument => 16,
            ExitCode::ErrNotFound => 17,
            ExitCode::ErrForbidden => 18,
            ExitCode::ErrInsufficientFunds => 19,
            ExitCode::ErrIllegalState => 20,
            ExitCode::ErrSerialization => 21,
            // ================================================================
            ExitCode::ErrPlaceholder => 1000,
        }
    }
}

impl TryFrom<i64> for ExitCode {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => ExitCode::Ok,
            1 => ExitCode::SysErrSenderInvalid,
            2 => ExitCode::SysErrSenderStateInvalid,
            3 => ExitCode::SysErrInvalidMethod,
            4 => ExitCode::SysErrInvalidParameters,
            5 => ExitCode::SysErrInvalidReceiver,
            6 => ExitCode::SysErrInsufficientFunds,
            7 => ExitCode::SysErrOutOfGas,
            8 => ExitCode::SysErrForbidden,
            9 => ExitCode::SysErrorIllegalActor,
            10 => ExitCode::SysErrorIllegalArgument,
            11 => ExitCode::SysErrSerialization,
            12 => ExitCode::SysErrorReserved3,
            13 => ExitCode::SysErrorReserved4,
            14 => ExitCode::SysErrorReserved5,
            15 => ExitCode::SysErrorReserved6,
            // ================================================================
            16 => ExitCode::ErrIllegalArgument,
            17 => ExitCode::ErrNotFound,
            18 => ExitCode::ErrForbidden,
            19 => ExitCode::ErrInsufficientFunds,
            20 => ExitCode::ErrIllegalState,
            21 => ExitCode::ErrSerialization,
            // ================================================================
            1000 => ExitCode::ErrPlaceholder,
            _ => return Err("Unknown exit code"),
        })
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self, *self as u64)
    }
}

impl error::Error for ExitCode {}

impl ExitCode {
    /// Check whether the execution of the VM is successful.
    pub fn is_success(self) -> bool {
        self == ExitCode::Ok
    }

    /// Check whether the execution of the VM is error.
    pub fn is_error(self) -> bool {
        !self.is_success()
    }

    /// Whether an exit code indicates a message send failure.
    /// A send failure means that the caller's CallSeqNum is not incremented and the caller has
    /// not paid gas fees for the message (because the caller doesn't exist or can't afford it).
    /// A receipt with send failure does not indicate that the message (or another one carrying the same CallSeqNum)
    /// could not apply in the future, against a different state.
    pub fn is_send_failure(self) -> bool {
        self == ExitCode::SysErrSenderInvalid || self == ExitCode::SysErrSenderStateInvalid
    }
}

// Implement CBOR serialization for ExitCode.
impl encode::Encode for ExitCode {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.i64(i64::from(*self))?.ok()
    }
}

// Implement CBOR deserialization for ExitCode.
impl<'b> decode::Decode<'b> for ExitCode {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        Self::try_from(d.i64()?).map_err(|err| decode::Error::Message(err))
    }
}
