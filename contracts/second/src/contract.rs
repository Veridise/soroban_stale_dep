use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SecondContract;

#[contractimpl]
impl SecondContract {
    pub fn some_op(_env: Env, a: i128, b: i128) -> i128 {
        a * b
    }
}