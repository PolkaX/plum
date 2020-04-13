// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_message::{MessageReceipt, UnsignedMessage};

///
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ExecutionResult {
    ///
    pub msg: UnsignedMessage,
    ///
    pub msg_receipt: MessageReceipt,
    ///
    pub error: String,
}

/// ExecutionResult JSON serialization/deserialization
pub mod json {
    use serde::{de, ser, Deserialize, Serialize};

    use plum_message::{
        message_receipt_json, unsigned_message_json, MessageReceipt, UnsignedMessage,
    };

    use super::ExecutionResult;

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonExecutionResultRef<'a> {
        #[serde(with = "unsigned_message_json")]
        msg: &'a UnsignedMessage,
        #[serde(with = "message_receipt_json")]
        #[serde(rename = "MsgRct")]
        msg_receipt: &'a MessageReceipt,
        error: &'a str,
    }

    /// JSON serialization
    pub fn serialize<S>(exe_result: &ExecutionResult, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        JsonExecutionResultRef {
            msg: &exe_result.msg,
            msg_receipt: &exe_result.msg_receipt,
            error: &exe_result.error,
        }
            .serialize(serializer)
    }

    /// JSON seq serialization
    pub fn serialize_seq<S>(
        exe_results: &[ExecutionResult],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        exe_results
            .iter()
            .map(|exe_result| JsonExecutionResultRef {
                msg: &exe_result.msg,
                msg_receipt: &exe_result.msg_receipt,
                error: &exe_result.error,
            })
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct JsonExecutionResult {
        #[serde(with = "unsigned_message_json")]
        msg: UnsignedMessage,
        #[serde(with = "message_receipt_json")]
        #[serde(rename = "MsgRct")]
        msg_receipt: MessageReceipt,
        error: String,
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<ExecutionResult, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        let JsonExecutionResult {
            msg,
            msg_receipt,
            error,
        } = JsonExecutionResult::deserialize(deserializer)?;
        Ok(ExecutionResult {
            msg,
            msg_receipt,
            error,
        })
    }

    /// JSON seq deserialization
    pub fn deserialize_seq<'de, D>(deserializer: D) -> Result<Vec<ExecutionResult>, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        let exe_results = <Vec<JsonExecutionResult>>::deserialize(deserializer)?;
        Ok(exe_results
            .into_iter()
            .map(|result| ExecutionResult {
                msg: result.msg,
                msg_receipt: result.msg_receipt,
                error: result.error,
            })
            .collect())
    }
}
