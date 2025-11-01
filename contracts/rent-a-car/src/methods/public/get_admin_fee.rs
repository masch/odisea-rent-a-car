use soroban_sdk::Env;

use crate::storage::admin::read_admin_fee;

pub fn get_admin_fee(env: &Env) -> i128 {
    read_admin_fee(env)
}
