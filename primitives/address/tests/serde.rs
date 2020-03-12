// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde_derive::{Deserialize, Serialize};

use plum_address::{address_cbor, address_json, set_network, Address, Network};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct CborAddress(#[serde(with = "address_cbor")] Address);

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct JsonAddress(#[serde(with = "address_json")] Address);

#[test]
fn address_default_serde() {
    let id_addr = Address::new_id_addr(12_512_063u64).unwrap();
    let cbor = serde_cbor::to_vec(&id_addr).unwrap();
    assert_eq!(cbor, [69, 0, 191, 214, 251, 5]);
    let out = serde_cbor::from_slice(&cbor).unwrap();
    assert_eq!(id_addr, out);
}

#[test]
fn address_cbor_serde() {
    let id_addr = CborAddress(Address::new_id_addr(12_512_063u64).unwrap());
    let cbor = serde_cbor::to_vec(&id_addr).unwrap();
    assert_eq!(cbor, [69, 0, 191, 214, 251, 5]);
    let out = serde_cbor::from_slice(&cbor).unwrap();
    assert_eq!(id_addr, out);
}

#[test]
fn address_json_serde() {
    let id_addr = JsonAddress(Address::new_id_addr(1024).unwrap());
    assert_eq!(id_addr.0.to_string(), "f01024");
    let json = serde_json::to_string(&id_addr).unwrap();
    assert_eq!(json, "\"f01024\"");
    let out = serde_json::from_str(&json).unwrap();
    assert_eq!(id_addr, out);

    unsafe { set_network(Network::Test) };

    let id_addr = JsonAddress(Address::new_id_addr(1024).unwrap());
    assert_eq!(id_addr.0.to_string(), "t01024");
    let json = serde_json::to_string(&id_addr).unwrap();
    assert_eq!(json, "\"t01024\"");
    let out = serde_json::from_str(&json).unwrap();
    assert_eq!(id_addr, out);
}
