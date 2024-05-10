#![cfg(test)]
extern crate std;

use crate::contract::{FirstContract, FirstContractClient};
use soroban_sdk::{Env, BytesN, Bytes};

mod second {
    soroban_sdk::contractimport!(
      file = "../../target/wasm32-unknown-unknown/release/second.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let client_addr = env.register_contract(None, FirstContract);
    let client = FirstContractClient::new(&env, &client_addr);
    let second_addr = env.register_contract_wasm(None, second::WASM);
    let result = client.consumer(&second_addr, &1, &0);
    assert_eq!(result, 0);
}
