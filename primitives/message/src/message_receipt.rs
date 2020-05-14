// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use minicbor::{decode, encode, Decoder, Encoder};
use serde::{Deserialize, Serialize};

use plum_bigint::{bigint_json, BigInt, BigIntRefWrapper, BigIntWrapper};

/// The receipt of applying message.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageReceipt {
    /// The exit code of VM.
    pub exit_code: u8,
    /// The return bytes.
    #[serde(with = "plum_bytes")]
    pub r#return: Vec<u8>,
    /// The used number of gas.
    #[serde(with = "bigint_json")]
    pub gas_used: BigInt,
}

// Implement CBOR serialization for MessageReceipt.
impl encode::Encode for MessageReceipt {
    fn encode<W: encode::Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.array(3)?
            .u8(self.exit_code)?
            .bytes(&self.r#return)?
            .encode(BigIntRefWrapper::from(&self.gas_used))?
            .ok()
    }
}

// Implement CBOR deserialization for MessageReceipt.
impl<'b> decode::Decode<'b> for MessageReceipt {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        let array_len = d.array()?;
        assert_eq!(array_len, Some(3));
        Ok(MessageReceipt {
            exit_code: d.u8()?,
            r#return: d.bytes()?.to_vec(),
            gas_used: d.decode::<BigIntWrapper>()?.into_inner(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_receipt_cbor_serde() {
        let receipt = MessageReceipt {
            exit_code: 127u8,
            r#return: b"ret".to_vec(),
            gas_used: BigInt::from(1_776_234),
        };
        let expected = vec![131, 24, 127, 67, 114, 101, 116, 68, 0, 27, 26, 106];

        let ser = minicbor::to_vec(&receipt).unwrap();
        assert_eq!(ser, expected);
        let de = minicbor::decode::<MessageReceipt>(&ser).unwrap();
        assert_eq!(de, receipt);
    }

    #[test]
    fn message_receipt_json_serde() {
        let receipt = MessageReceipt {
            exit_code: 127u8,
            r#return: b"ret".to_vec(),
            gas_used: BigInt::from(1_776_234),
        };
        let expected = "{\"ExitCode\":127,\"Return\":\"cmV0\",\"GasUsed\":\"1776234\"}";

        let ser = serde_json::to_string(&receipt).unwrap();
        assert_eq!(ser, expected);
        let de = serde_json::from_str::<MessageReceipt>(&ser).unwrap();
        assert_eq!(de, receipt);
    }
}
