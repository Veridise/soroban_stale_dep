use soroban_sdk::{contract, contractimpl, Env, Address};

// for an interface to call the second constract
mod second {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/second.wasm"
    );
}

#[contract]
pub struct FirstContract;

#[contractimpl]
impl FirstContract {
    pub fn consumer(env: Env, contract_addr: Address, a: i128, b: i128) -> i128 {
        let client = second::Client::new(&env, &contract_addr);
        client.some_op(&a, &b)
    }
}
