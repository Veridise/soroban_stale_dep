use soroban_sdk::{contract, contractimpl, Env};

mod second {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/second.wasm"
    );
}

#[contract]
pub struct FirstContract;

#[contractimpl]
impl FirstContract {
    pub fn consumer(_env:Env) -> u32 {
        second::VerySeriousEnum::Warning as u32
    }
}
