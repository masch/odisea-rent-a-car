use soroban_sdk::Env;

use crate::storage::admin::read_admin_balance;

pub fn get_admin_fee_to_withdraw(env: &Env) -> i128 {
    read_admin_balance(env)
}
