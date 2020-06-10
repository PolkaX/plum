// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

use cid::Cid;
use plum_address::Address;
use plum_bigint::bigint_json;
use plum_piece::PaddedPieceSize;
use plum_types::{ChainEpoch, TokenAmount};

// Note: Deal Collateral is only released and returned to clients and miners
// when the storage deal stops counting towards power. In the current iteration,
// it will be released when the sector containing the storage deals expires,
// even though some storage deals can expire earlier than the sector does.
// Collaterals are denominated in PerEpoch to incur a cost for self dealing or
// minimal deals that last for a long time.
// Note: ClientCollateralPerEpoch may not be needed and removed pending future confirmation.
// There will be a Minimum value for both client and provider deal collateral.
///
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealProposal {
    #[serde(rename = "PieceCID")]
    pub piece_cid: Cid, // CommP
    pub piece_size: PaddedPieceSize,
    pub verified_deal: bool,
    pub client: Address,
    pub provider: Address,

    // Nominal start epoch. Deal payment is linear between StartEpoch and EndEpoch,
    // with total amount StoragePricePerEpoch * (EndEpoch - StartEpoch).
    // Storage deal must appear in a sealed (proven) sector no later than StartEpoch,
    // otherwise it is invalid.
    pub start_epoch: ChainEpoch,
    pub end_epoch: ChainEpoch,
    #[serde(with = "bigint_json")]
    pub storage_price_per_epoch: TokenAmount,

    #[serde(with = "bigint_json")]
    pub provider_collateral: TokenAmount,
    #[serde(with = "bigint_json")]
    pub client_collateral: TokenAmount,
}

///
#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealState {
    pub sector_start_epoch: ChainEpoch, // -1 if not yet included in proven sector
    pub last_updated_epoch: ChainEpoch, // -1 if deal state never updated
    pub slash_epoch: ChainEpoch,        // -1 if deal never slashed
}
