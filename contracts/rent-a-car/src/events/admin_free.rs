use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn set_admin_fee(env: &Env, admin: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "set_admin_fee"), admin.clone());

    env.events().publish(topics, amount);
}

pub(crate) fn withdraw_admin_fee(env: &Env, admin: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "withdraw_admin_fee"), admin.clone());

    env.events().publish(topics, amount);
}
