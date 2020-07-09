// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use crate::gas_v0::PricelistV0;
use lazy_static::lazy_static;
use plum_crypto::SignatureType;
use plum_piece::PieceInfo;
use plum_sector::{RegisteredSealProof, SealVerifyInfo, WindowPoStVerifyInfo};
use plum_types::{ChainEpoch, Gas, MethodNum, TokenAmount};
use std::collections::HashMap;

///
pub type Size = usize;

/// Pricelist provides prices for operations in the VM.
///
/// Note: this interface should be APPEND ONLY since last chain checkpoint
pub trait Pricelist {
    /// OnChainMessage returns the gas used for storing a message of a given size in the chain.
    fn on_chain_message(&self, msg_size: Size) -> Gas;
    /// OnChainReturnValue returns the gas used for storing the response of a message in the chain.
    fn on_chain_return_value(&self, data_size: Size) -> Gas;

    /// OnMethodInvocation returns the gas used when invoking a method.
    fn on_method_invocation(&self, value: TokenAmount, method_num: MethodNum) -> Gas;

    /// OnIpldGet returns the gas used for storing an object
    fn on_ipld_get(&self, data_size: Size) -> Gas;
    /// OnIpldPut returns the gas used for storing an object
    fn on_ipld_put(&self, data_size: Size) -> Gas;

    /// OnCreateActor returns the gas used for creating an actor
    fn on_create_actor(&self) -> Gas;
    /// OnDeleteActor returns the gas used for deleting an actor
    fn on_delete_actor(&self) -> Gas;

    ///
    fn on_verify_signature(&self, sig_type: SignatureType, plan_text_size: Size) -> Gas;
    ///
    fn on_hashing(&self, data_size: Size) -> Gas;
    ///
    fn on_compute_unsealed_sector_cid(
        &self,
        proof_type: RegisteredSealProof,
        pieces: Vec<PieceInfo>,
    ) -> Gas;
    ///
    fn on_verify_seal(&self, info: SealVerifyInfo) -> Gas;
    ///
    fn on_verify_post(&self, info: WindowPoStVerifyInfo) -> Gas;
    ///
    fn on_verify_consensus_fault(&self) -> Gas;
}

lazy_static! {
    /// TODO: if this won't be changed once initialized, use const instead.
    pub static ref PRICES: HashMap<ChainEpoch, PricelistV0> = {
        let mut m = HashMap::new();
        m.insert(
            0,
            PricelistV0 {
                on_chain_message_base: 0.into(),
                on_chain_message_per_byte: 2.into(),
                on_chain_return_value_per_byte: 8.into(),
                send_base: 5.into(),
                send_transfer_funds: 5.into(),
                send_invoke_method: 10.into(),
                ipld_get_base: 10.into(),
                ipld_get_per_byte: 1.into(),
                ipld_put_base: 20.into(),
                ipld_put_per_byte: 2.into(),
                create_actor_base: 40.into(), // IPLD put + 20
                create_actor_extra: 500.into(),
                delete_actor: (-500).into(), // -createActorExtra

                // NOTE: handled inside on_verify_signature()
                // Dragons: this cost is not persistable, create a LinearCost{a,b} struct that has a `.Cost(x) -> ax + b`
                // verify_signature: map[crypto.SigType]func(int64) int64{
                // crypto.SigTypeBLS:       func(x int64) int64 { return 3*x + 2 },
                // crypto.SigTypeSecp256k1: func(x int64) int64 { return 3*x + 2 },
                // },

                hashing_base: 5.into(),
                hashing_per_byte: 2.into(),
                compute_unsealed_sector_cid_base: 100.into(),
                verify_seal_base: 2000.into(),
                verify_post_base: 700.into(),
                verify_consensus_fault: 10.into(),
            },
        );
        m
    };
}

/// PricelistByEpoch finds the latest prices for the given epoch
pub fn pricelist_by_epoch(epoch: ChainEpoch) -> &'static PricelistV0 {
    // since we are storing the prices as map or epoch to price
    // we need to get the price with the highest epoch that is lower or equal to the `epoch` arg
    let best_epoch = PRICES
        .keys()
        .filter(|e| **e > 0 && **e <= epoch)
        .max()
        .unwrap_or(&0);

    PRICES.get(best_epoch).expect(&format!(
        "bad setup: no gas prices available for epoch {}",
        epoch
    ))
}
