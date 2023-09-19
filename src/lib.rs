mod utils;

use anychain_core::{hex, libsecp256k1, PublicKey};
use anychain_ethereum::{EthereumAddress, EthereumFormat, EthereumPublicKey};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn trim0x(s: &str) -> String {
    // str to lower case
    let s = s.to_lowercase();
    // remove 0x prefix
    let s = s.trim_start_matches("0x");
    s.to_string()
}

fn address_from_public_key_internal(public_key: libsecp256k1::PublicKey) -> EthereumAddress {
    let public_key = EthereumPublicKey::from_secp256k1_public_key(public_key);
    public_key.to_address(&EthereumFormat::Standard).unwrap()
}
#[wasm_bindgen]
pub fn address_from_public_key(public_key: String) -> String {
    let public_key = {
        let public_key = trim0x(&public_key);
        let public_key = hex::decode(public_key).unwrap();
        // parse_slice can handle both compressed and uncompressed public key
        // therefore we don't need to check the length and prefix of the public key
        libsecp256k1::PublicKey::parse_slice(&public_key, None).unwrap()
    };

    format!("{}", address_from_public_key_internal(public_key))
}
