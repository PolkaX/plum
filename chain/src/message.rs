// Copright 2019 chainnet.tech

use crate::{BigInt, Address};

pub struct Message {
    to: Address,
    from: Address,
    nonce: u64,
    value: BigInt,
    gas_price: BigInt,
    gas_limit: BigInt,
    method: u64,
    params: Vec<u8>,
}
