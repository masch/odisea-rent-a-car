use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub(crate) fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;

    env.storage().instance().has(&key)
}

pub(crate) fn read_admin(env: &Env) -> Address {
    let key = DataKey::Admin;

    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn write_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;

    env.storage().instance().set(&key, admin);
}
