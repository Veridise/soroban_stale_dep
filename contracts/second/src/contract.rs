use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contract]
pub struct SecondContract;

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VerySeriousEnum {
    Warning = 5,
    Danger = 2
}

#[contractimpl]
impl SecondContract {
    // Just a stub, so Soroban will not consider this contract empty or something.
    pub fn some_op(_env: Env, a: i128, b: i128) -> i128 {
        a * b
    }
}