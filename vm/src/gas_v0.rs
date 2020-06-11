use crate::gas::{Gas, Pricelist, Size};
use plum_actor::MethodSend;
use plum_crypto::SignatureType;
use plum_piece::PieceInfo;
use plum_sector::{RegisteredProof, SealVerifyInfo, WindowPoStVerifyInfo};
use plum_types::{MethodNum, TokenAmount};

///
#[derive(Debug, Clone)]
pub struct PricelistV0 {
    ////////////////////////////////////////////////////////////////////////////
    // System operations
    ///////////////////////////////////////////////////////////////////////////
    /// Gas cost charged to the originator of an on-chain message (regardless of
    /// whether it succeeds or fails in application) is given by:
    ///   OnChainMessageBase + len(serialized message)*OnChainMessagePerByte
    /// Together, these account for the cost of message propagation and validation,
    /// up to but excluding any actual processing by the VM.
    /// This is the cost a block producer burns when including an invalid message.
    pub on_chain_message_base: Gas,
    pub on_chain_message_per_byte: Gas,

    /// Gas cost charged to the originator of a non-nil return value produced
    /// by an on-chain message is given by:
    ///   len(return value)*OnChainReturnValuePerByte
    pub on_chain_return_value_per_byte: Gas,

    /// Gas cost for any message send execution(including the top-level one
    /// initiated by an on-chain message).
    /// This accounts for the cost of loading sender and receiver actors and
    /// (for top-level messages) incrementing the sender's sequence number.
    /// Load and store of actor sub-state is charged separately.
    pub send_base: Gas,

    /// Gas cost charged, in addition to SendBase, if a message send
    /// is accompanied by any nonzero currency amount.
    /// Accounts for writing receiver's new balance (the sender's state is
    /// already accounted for).
    pub send_transfer_funds: Gas,

    /// Gas cost charged, in addition to SendBase, if a message invokes
    /// a method on the receiver.
    /// Accounts for the cost of loading receiver code and method dispatch.
    pub send_invoke_method: Gas,

    /// Gas cost (Base + len*PerByte) for any Get operation to the IPLD store
    /// in the runtime VM context.
    pub ipld_get_base: Gas,
    pub ipld_get_per_byte: Gas,

    /// Gas cost (Base + len*PerByte) for any Put operation to the IPLD store
    /// in the runtime VM context.
    ///
    /// Note: these costs should be significantly higher than the costs for Get
    /// operations, since they reflect not only serialization/deserialization
    /// but also persistent storage of chain data.
    pub ipld_put_base: Gas,
    pub ipld_put_per_byte: Gas,

    // Gas cost for creating a new actor (via InitActor's Exec method).
    //
    // Note: this costs assume that the extra will be partially or totally refunded while
    // the base is covering for the put.
    pub create_actor_base: Gas,
    pub create_actor_extra: Gas,

    /// Gas cost for deleting an actor.
    ///
    /// Note: this partially refunds the create cost to incentivise the deletion of the actors.
    pub delete_actor: Gas,

    /// verify_signature map[crypto.SigType]func(len : Gas) int64,
    pub hashing_base: Gas,
    pub hashing_per_byte: Gas,

    pub compute_unsealed_sector_cid_base: Gas,
    pub verify_seal_base: Gas,
    pub verify_post_base: Gas,
    pub verify_consensus_fault: Gas,
}

impl Pricelist for PricelistV0 {
    fn on_chain_message(&self, msg_size: Size) -> Gas {
        self.on_chain_message_base + self.on_chain_message_per_byte * msg_size as Gas
    }

    fn on_chain_return_value(&self, data_size: Size) -> Gas {
        data_size as Gas * self.on_chain_return_value_per_byte
    }

    fn on_method_invocation(&self, value: TokenAmount, method_num: MethodNum) -> Gas {
        let mut invocation = self.send_base;
        if value != 0.into() {
            invocation += self.send_transfer_funds;
        }
        if method_num != MethodSend {
            invocation += self.send_invoke_method;
        }
        invocation
    }

    fn on_ipld_get(&self, data_size: Size) -> Gas {
        self.ipld_get_base + data_size as Gas * self.ipld_get_per_byte
    }

    fn on_ipld_put(&self, data_size: Size) -> Gas {
        self.ipld_put_base + data_size as Gas * self.ipld_put_per_byte
    }

    fn on_create_actor(&self) -> Gas {
        self.create_actor_base + self.create_actor_extra
    }

    fn on_delete_actor(&self) -> Gas {
        self.delete_actor
    }

    fn on_verify_signature(&self, sig_type: SignatureType, plan_text_size: Size) -> Gas {
        let gas_for = |s: Size| 3 * s as Gas + 2;
        match sig_type {
            SignatureType::Bls => gas_for(plan_text_size),
            SignatureType::Secp256k1 => gas_for(plan_text_size),
        }
    }

    fn on_hashing(&self, data_size: Size) -> Gas {
        self.hashing_base + data_size as Gas * self.hashing_per_byte
    }

    fn on_compute_unsealed_sector_cid(
        &self,
        _proof_type: RegisteredProof,
        _pieces: Vec<PieceInfo>,
    ) -> Gas {
        // TODO: this needs more cost tunning, check with @lotus
        self.compute_unsealed_sector_cid_base
    }

    fn on_verify_seal(&self, _info: SealVerifyInfo) -> Gas {
        // TODO: this needs more cost tunning, check with @lotus
        self.verify_seal_base
    }

    fn on_verify_post(&self, _info: WindowPoStVerifyInfo) -> Gas {
        // TODO: this needs more cost tunning, check with @lotus
        self.verify_post_base
    }

    fn on_verify_consensus_fault(&self) -> Gas {
        self.verify_consensus_fault
    }
}
