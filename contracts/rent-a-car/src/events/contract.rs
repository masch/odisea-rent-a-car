use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn contract_initialized(env: &Env, admin: &Address, token: &Address, admin_fee: &i128) {
    let topics = (Symbol::new(env, "contract_initialized"),);

    env.events().publish(topics, (admin, token, admin_fee));
}
