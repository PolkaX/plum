// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! The Common types of primitives.

#![deny(missing_docs)]

use cid::Cid;
use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_bigint::{bigint_json, BigInt, BigIntWrapper};
use plum_bytes::Bytes;

mod constants;

pub use self::constants::*;

/// A sequential number assigned to an actor when created by the InitActor.
/// This ID is embedded in ID-type addresses.
pub type ActorId = u64;
///
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Actor {
    ///
    pub code: Cid,
    ///
    pub head: Cid,
    ///
    pub nonce: u64,
    ///
    #[serde(with = "bigint_json")]
    pub balance: BigInt,
}

// Implement CBOR serialization for Actor.
impl encode::Encode for Actor {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(4)?
            .encode(&self.code)?
            .encode(&self.head)?
            .u64(self.nonce)?
            .encode(BigIntWrapper::from(self.balance.clone()))?
            .ok()
    }
}

// Implement CBOR deserialization for Actor.
impl<'b> decode::Decode<'b> for Actor {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(4));
        Ok(Actor {
            code: d.decode::<Cid>()?,
            head: d.decode::<Cid>()?,
            nonce: d.u64()?,
            balance: d.decode::<BigIntWrapper>()?.into_inner(),
        })
    }
}

/// Epoch number of the chain state, which acts as a proxy for time within the VM.
pub type ChainEpoch = i64;
///
pub type EpochDuration = u64;

/// MethodNum is an integer that represents a particular method
/// in an actor's function table. These numbers are used to compress
/// invocation of actor code, and to decouple human language concerns
/// about method names from the ability to uniquely refer to a particular
/// method.
///
/// Consider MethodNum numbers to be similar in concerns as for
/// offsets in function tables (in programming languages), and for
/// tags in ProtocolBuffer fields. Tags in ProtocolBuffers recommend
/// assigning a unique tag to a field and never reusing that tag.
/// If a field is no longer used, the field name may change but should
/// still remain defined in the code to ensure the tag number is not
/// reused accidentally. The same should apply to the MethodNum
/// associated with methods in Filecoin VM Actors.
pub type MethodNum = u64;

/// TokenAmount is an amount of Filecoin tokens. This type is used within
/// the VM in message execution, to account movement of tokens, payment
/// of VM gas, and more.
///
/// BigInt types are aliases rather than new types because the latter introduce incredible amounts of noise converting to
/// and from types in order to manipulate values. We give up some type safety for ergonomics.
pub type TokenAmount = BigInt;

/// Randomness is a string of random bytes
pub type Randomness = Bytes;

///
pub type DealId = u64;
/// BigInt types are aliases rather than new types because the latter introduce incredible amounts of noise converting to
/// and from types in order to manipulate values. We give up some type safety for ergonomics.
/// units: byte-epochs
pub type DealWeight = BigInt;

///
pub type Gas = BigInt;
