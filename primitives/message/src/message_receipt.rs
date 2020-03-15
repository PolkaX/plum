// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser};

use plum_bigint::BigInt;

/// The receipt of applying message.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MessageReceipt {
    /// The exit code of VM.
    pub exit_code: u8,
    /// The return bytes.
    pub ret: Vec<u8>,
    /// The used number of gas.
    pub gas_used: BigInt,
}

impl ser::Serialize for MessageReceipt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for MessageReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::cbor::deserialize(deserializer)
    }
}

/// MessageReceipt CBOR serialization/deserialization.
pub mod cbor {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_bigint::{bigint_cbor, BigInt};

    use super::MessageReceipt;

    #[derive(Serialize)]
    struct TupleMessageReceiptRef<'a>(
        &'a u8,
        #[serde(with = "serde_bytes")] &'a Vec<u8>,
        #[serde(with = "bigint_cbor")] &'a BigInt,
    );

    /// CBOR serialization.
    pub fn serialize<S>(receipt: &MessageReceipt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        TupleMessageReceiptRef(&receipt.exit_code, &receipt.ret, &receipt.gas_used)
            .serialize(serializer)
    }

    #[derive(Deserialize)]
    struct TupleMessageReceipt(
        u8,
        #[serde(with = "serde_bytes")] Vec<u8>,
        #[serde(with = "bigint_cbor")] BigInt,
    );

    /// CBOR deserialization.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<MessageReceipt, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let TupleMessageReceipt(exit_code, ret, gas_used) =
            TupleMessageReceipt::deserialize(deserializer)?;
        Ok(MessageReceipt {
            exit_code,
            ret,
            gas_used,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::MessageReceipt;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct CborMessageReceipt(#[serde(with = "super::cbor")] MessageReceipt);

    #[test]
    fn message_receipt_cbor_serde() {
        let receipt = CborMessageReceipt(MessageReceipt {
            exit_code: 127u8,
            ret: b"ret".to_vec(),
            gas_used: 1_776_234.into(),
        });
        let expected = [131, 24, 127, 67, 114, 101, 116, 68, 0, 27, 26, 106];

        let ser = serde_cbor::to_vec(&receipt).unwrap();
        assert_eq!(ser, &expected[..]);
        let de = serde_cbor::from_slice::<CborMessageReceipt>(&ser).unwrap();
        assert_eq!(de, receipt);
    }
}
