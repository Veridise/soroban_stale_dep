#![cfg(test)]
extern crate std;

use crate::contract::{FirstContract, FirstContractClient};
use soroban_sdk::{Env};

// we need this to deploy the `second` contract and pass its address
// to the `first` contract
mod second {
    soroban_sdk::contractimport!(
      file = "../../target/wasm32-unknown-unknown/release/second.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let first_addr = env.register_contract(None, FirstContract);
    let first = FirstContractClient::new(&env, &first_addr);
    let second_addr = env.register_contract_wasm(None, second::WASM);
    let result = first.consumer(&second_addr, &1, &0);
    assert_eq!(result, 1);
}
