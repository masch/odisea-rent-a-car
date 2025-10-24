use crate::interfaces::contract::RentACarContractTrait;
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct RentACarContract;

pub const ADMIN_KEY: &Symbol = &symbol_short!("ADMIN");
pub const TOKEN_KEY: &Symbol = &symbol_short!("TOKEN");

#[contractimpl]
impl RentACarContractTrait for RentACarContract {
    fn __constructor(env: &Env, admin: Address, token: Address) {
        env.storage().instance().set(ADMIN_KEY, &admin);
        env.storage().instance().set(TOKEN_KEY, &token);
    }

    fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(ADMIN_KEY).unwrap()
    }
}
