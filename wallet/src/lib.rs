use address::{Account, Address, Display, Network};
use crypto::{key_types, KeyTypeId};
use error::Error;
use keystore::{KeyPair, Store};
use parking_lot::RwLock;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    sync::Arc,
};

mod address;
pub mod crypto;
mod error;
mod keystore;

//
// Generate
// Sign
// Remove
//walletNew,
//walletList,
//walletBalance,
//walletexport
/// wallet pointer
pub type WalletPtr = Arc<RwLock<Wallet>>;
const KEYSTORE_PATH: &str = "/.plum/keystore/";

pub type Result<T> = std::result::Result<T, Error>;
pub struct Wallet {}

impl Wallet {
    pub fn new_address(key_type: KeyTypeId) {
        let home = std::env::var("HOME").unwrap();
        let store = Store {
            path: PathBuf::from_str(&(home.as_str().to_owned() + KEYSTORE_PATH)).unwrap(),
            additional: HashMap::new(),
        };
        Store::open(store.path.clone()).unwrap();
        let pair = store.generate_key(key_type).unwrap();
        println!("{}\n", pair.to_string(key_type, Network::Testnet));
    }

    pub fn wallet_list(keystore_path: Option<String>) {
        let home = std::env::var("HOME").unwrap();
        let path = match keystore_path {
            Some(p) => p,
            None => home + &KEYSTORE_PATH.to_string(),
        };
        let keystore_path = PathBuf::from_str(path.as_str()).unwrap();
        if !keystore_path.exists() {
            println!("No such file: {:?}", keystore_path.clone());
        }

        let store = Store::open(keystore_path.clone()).unwrap();
        let entries = fs::read_dir(keystore_path).unwrap();
        for file in entries {
            let file_name = file.unwrap().file_name();
            match hex::decode(file_name.to_str().unwrap()) {
                Ok(ref name) if name.len() > 4 => {
                    let type_name = std::str::from_utf8(&name[0..4]).unwrap();
                    let key_type = KeyTypeId::try_from(type_name).unwrap();
                    let public = &name[4..];

                    println!("{}", pubkey_to_address(public.to_vec(), key_type, Network::Testnet));
                    println!("pubkey: {}\n\n", hex::encode(&public));
                }
                _ => continue,
            }
        }
    }

    pub fn export(pubkey: String) {
        let home = std::env::var("HOME").unwrap();
        let path = home + &KEYSTORE_PATH.to_string();
        let keystore_path = PathBuf::from_str(path.as_str()).unwrap();
        if !keystore_path.exists() {
            println!("No such file: {:?}", keystore_path.clone());
        }

        let store = Store::open(keystore_path.clone()).unwrap();
        let entries = fs::read_dir(keystore_path).unwrap();
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
                        let key_type = KeyTypeId::try_from(type_name).unwrap();
                        let public = &hex_name[4..];
                        let mut file = File::open(path + name).unwrap();
                        let mut file_copy = file.try_clone().unwrap();
                        let mut contents = String::new();
                        file_copy.read_to_string(&mut contents).unwrap();
                        let privkey = &contents[1..contents.len() - 1];
                        println!("{}", pubkey_to_address(public.to_vec(), key_type, Network::Testnet));
                        println!("private_key: {}\n\n",privkey);
                    }
                    Err(e) => println!("{}", e),
                }
                return;
            } else {
                continue;
            }
        }
    }
    pub fn import(key_type: KeyTypeId, privkey: String) {
        let privkey = hex::decode(privkey).unwrap();
        let home = std::env::var("HOME").unwrap();
        let path = home + &KEYSTORE_PATH.to_string();
        let store = Store {
            path: PathBuf::from_str(&path).unwrap(),
            additional: HashMap::new(),
        };
        Store::open(store.path.clone()).unwrap();
        let pair = store.import_key(key_type, privkey.as_slice()).unwrap();
        println!("{}\n", pair.to_string(key_type, Network::Testnet));
    }
}

fn pubkey_to_address(pubkey: Vec<u8>, key_type: KeyTypeId, net: Network) -> String {
    match key_type {
        key_types::BLS => {
            let addr: Address = Account::BLS(pubkey).try_into().unwrap();
            format!(
                "address: {}\ntype: {}",
                addr.display(net),
                "bls"
            )
        },
        key_types::SECP256K1 => {
            let addr: Address =
                Account::SECP256K1(pubkey).try_into().unwrap();
            format!(
                "address: {}\ntype: {}",
                addr.display(net),
                "secp256k1"
            )
        }
        _ => unreachable!("only bls,secp256k1"),
    }
}