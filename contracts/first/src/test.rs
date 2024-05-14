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
    assert_eq!(second::VerySeriousEnum::Warning as u32, 5);
}
