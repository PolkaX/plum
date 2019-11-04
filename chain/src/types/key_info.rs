// Copyright 2019 chainnet.tech

use crate::types::Error;

pub struct KeyInfo {
    type_: String,
    private_key: Vec<u8>,
}

pub trait KeyStore {
    fn list() -> (Vec<String>, Error);
    fn get(_: String) -> (KeyInfo, Error);
    fn put(_: String, _: KeyInfo) -> Error;
    fn delete(_: String) -> Error;
}
