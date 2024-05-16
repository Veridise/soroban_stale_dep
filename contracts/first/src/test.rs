#![cfg(test)]
extern crate std;

use crate::contract::{FirstContract, FirstContractClient};
use soroban_sdk::{Env};

mod first {
    soroban_sdk::contractimport!(
      file = "../../target/wasm32-unknown-unknown/release/first.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let first = FirstContractClient::new(&env, &env.register_contract(None, FirstContract {}));
    assert_eq!(first.consumer(), 5);
}
