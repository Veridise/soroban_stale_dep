use soroban_sdk::{contract, contractimpl, Env, Address};

// for an interface to call the second contract
mod second {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/second.wasm"
    );
}

#[contract]
pub struct FirstContract;

#[contractimpl]
impl FirstContract {
    // we want to demonstrate that the value used is outdated
    pub fn consumer(env:Env) -> u32 {
        second::VerySeriousEnum::Warning as u32
    }
}
