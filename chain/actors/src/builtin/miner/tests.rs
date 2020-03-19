use std::fmt::Debug;
use std::convert::TryFrom;
use serde::{Serialize, de::DeserializeOwned};

use cid::Cid;
use plum_address::Address;
use hex_literal::hex;

use super::*;
use super::state::*;
use super::actor::*;

use crate::abi::bitfield::BitField;


fn test_cbor<T: Serialize+DeserializeOwned+Debug+Eq>(obj: T, expect: Vec<u8>) {
    let v = serde_cbor::to_vec(&obj).unwrap();
    assert_eq!(v, expect);
    let out = serde_cbor::from_slice(&v).unwrap();
    assert_eq!(obj, out)
}

fn miner_info() -> MinerInfo {
    MinerInfo {
        owner: Address::new_id_addr(2).unwrap(),
        worker: Address::new_id_addr(3).unwrap(),
        pending_worker_key: None,
        peer_id: String::from_utf8_lossy(&hex!("DEAD")).to_string(),
        sector_size: 4,
    }
}

fn miner_info2() -> MinerInfo {
    MinerInfo {
        owner: Address::new_id_addr(2).unwrap(),
        worker: Address::new_id_addr(3).unwrap(),
        pending_worker_key: Some(WorkerKeyChange { new_worker: Address::new_id_addr(2).unwrap(), effective_at: 1.into() }),
        peer_id: String::from_utf8_lossy(&hex!("DEAD")).to_string(),
        sector_size: 4,
    }
}

#[test]
fn test_cbor_mineractorstate() {
    let c = Cid::try_from(hex!("015501020001").as_ref()).unwrap();
    let mut bitfield = BitField::new();
    bitfield.insert(2);
    bitfield.insert(7);
    let state =  State {
        precommitted_sectors: Cid::try_from(hex!("015501020001").as_ref()).unwrap(),
        sectors: Cid::try_from(hex!("015501020002").as_ref()).unwrap(),
        fault_set: bitfield.clone(),
        proving_set: Cid::try_from(hex!("015501020003").as_ref()).unwrap(),
        info: miner_info(),
        post_state: PoStState {
            proving_period_start: 1.into(),
            num_consecutive_failures: 2,
        }
    };

    test_cbor(state, hex!("86d82a4700015501020001d82a470001550102000243504a01d82a470001550102000385420002420003f662dead04820102").to_vec());

    let state =  State {
        precommitted_sectors: Cid::try_from(hex!("015501020001").as_ref()).unwrap(),
        sectors: Cid::try_from(hex!("015501020002").as_ref()).unwrap(),
        fault_set: bitfield,
        proving_set: Cid::try_from(hex!("015501020003").as_ref()).unwrap(),
        info: miner_info2(),
        post_state: PoStState {
            proving_period_start: 1.into(),
            num_consecutive_failures: 2,
        }
    };

    test_cbor(state, hex!("86d82a4700015501020001d82a470001550102000243504a01d82a470001550102000385420002420003824200020162dead04820102").to_vec());
}

#[test]
fn miner_cbor_sector_info() {
    let info = SectorPreCommitInfo {
        sector: 1,
        sealed_cid: Cid::try_from(hex!("015501020001").as_ref()).unwrap(),
        seal_epoch: 2.into(),
        deal_ids: vec![3],
        expiration: 4.into()
    };

    let onchain_info = SectorPreCommitOnChainInfo {
        info: info.clone(),
        precommit_deposit: 1.into(),
        precommit_epoch: 2.into()
    };

    test_cbor(onchain_info, hex!("838501d82a47000155010200010281030442000102").to_vec());

    let sector_info = SectorOnChainInfo {
        info,
        activation_epoch: 1.into(),
        deal_weight: 2.into(),
        pledge_requirement: 3.into(),
        declared_fault_epoch: 4.into(),
        declared_fault_duration: 5.into()
    };

    test_cbor(sector_info, hex!("868501d82a470001550102000102810304014200024200030405").to_vec());
}

#[test]
fn test_cbor_miner_info() {
    let mut info = miner_info();
    test_cbor(info.clone(), hex!("85420002420003f662dead04").to_vec());

    info.pending_worker_key = Some(WorkerKeyChange {
        new_worker: Address::new_id_addr(6).unwrap(),
        effective_at: 5.into()
    });
    test_cbor(info, hex!("85420002420003824200060562dead04").to_vec());
}

#[test]
fn test_cbor_proof() {
    let proof = RegisteredProof::StackedDRG512MiBPoSt;
    test_cbor(proof, hex!("08").to_vec());
}