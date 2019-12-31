// Copyright 2019 PolkaX Authors. Licensed under GPL-3.0.

//!

#![deny(missing_docs)]

mod crypto;
mod error;
mod keypair;
mod keystore;

use std::{
    convert::{TryFrom, TryInto},
    fs::{self, File},
    io::Read,
    path::PathBuf,
    str::FromStr,
};

use address::{Account, Address, KeyType, Network};

use self::error::{Result, WalletError};
use self::keystore::Store;

const KEYSTORE_PATH: &str = "/.plum/keystore/";
const NET_TYPE: Network = Network::Test;

pub struct Wallet;
impl Wallet {
    /// generate address by type
    pub fn new_address(key_type: KeyType) {
        let store = check_keystore_path();
        let pair = store.generate_key(key_type).unwrap();
        println!("{}\n", pair.display(key_type, NET_TYPE).unwrap());
    }

    /// list all address-info in keystore
    pub fn wallet_list() {
        let store = check_keystore_path();
        let _ = store.open().unwrap();
        let entries = fs::read_dir(store.path).unwrap();
        for file in entries {
            let file_name = file.unwrap().file_name();
            match hex::decode(file_name.to_str().unwrap()) {
                Ok(ref name) if name.len() > 4 => {
                    let type_name = std::str::from_utf8(&name[0..4]).unwrap();
                    let key_type = KeyType::try_from(type_name).unwrap();
                    let public = &name[4..];

                    println!("{}", pubkey_to_address(public.to_vec(), key_type, NET_TYPE));
                    println!("pubkey: {}\n\n", hex::encode(&public));
                }
                _ => continue,
            }
        }
    }

    /// export the address-info by public key
    pub fn export(pubkey: String) {
        let store = check_keystore_path();
        let entries = fs::read_dir(store.path.clone()).unwrap();
        let path = store.path.to_str().unwrap();
        for file in entries {
            let file_name = file.unwrap().file_name();
            let name = file_name.to_str().unwrap();
            if name.contains(&pubkey) {
                match hex::decode(name) {
                    Ok(ref hex_name) => {
                        if hex_name.len() < 4 {
                            println!("Invalid public key:{}", pubkey);
                            return;
                        }
                        let type_name = std::str::from_utf8(&hex_name[0..4]).unwrap();
                        let key_type = KeyType::try_from(type_name).unwrap();
                        let public = &hex_name[4..];
                        let file = File::open(path.to_owned() + name).unwrap();
                        let mut file_copy = file.try_clone().unwrap();
                        let mut contents = String::new();
                        file_copy.read_to_string(&mut contents).unwrap();
                        let privkey = &contents[1..contents.len() - 1];
                        println!("{}", pubkey_to_address(public.to_vec(), key_type, NET_TYPE));
                        println!("private_key: {}\n\n", privkey);
                    }
                    Err(e) => println!("{}", e),
                }
                return;
            } else {
                continue;
            }
        }
    }

    /// import account by type and private-key
    pub fn import(key_type: KeyType, privkey: String) {
        let store = check_keystore_path();
        let privkey = hex::decode(privkey).unwrap();
        let pair = store.import_key(key_type, privkey.as_slice()).unwrap();
        println!("{}\n", pair.display(key_type, Network::Test).unwrap());
    }
}

fn pubkey_to_address(pubkey: Vec<u8>, key_type: KeyType, net: Network) -> String {
    match key_type {
        key_types::BLS => {
            let addr: Address = Account::BLS(pubkey).try_into().unwrap();
            format!("address: {}\ntype: {}", addr.encode(net).unwrap(), "bls")
        }
        key_types::SECP256K1 => {
            let addr: Address = Account::SECP256K1(pubkey).try_into().unwrap();
            format!(
                "address: {}\ntype: {}",
                addr.encode(net).unwrap(),
                "secp256k1"
            )
        }
        _ => unreachable!("only bls,secp256k1"),
    }
}

fn check_keystore_path() -> Store {
    let home = std::env::var("HOME").unwrap();
    let store = Store {
        path: PathBuf::from_str(&(home.as_str().to_owned() + KEYSTORE_PATH)).unwrap(),
    };
    if !store.path.exists() {
        println!("No such file: {:?}", store.path.clone());
    }
    let _ = store.open().unwrap();
    store
}
