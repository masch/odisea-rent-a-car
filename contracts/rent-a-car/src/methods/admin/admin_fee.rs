use crate::{
    events,
    methods::token::token::token_transfer,
    storage::{
        admin::{read_admin, read_admin_balance, write_admin_balance, write_admin_fee},
        contract_balance::read_contract_balance,
        types::error::Error,
    },
};
use soroban_sdk::Env;

pub fn set_admin_fee(env: &Env, amount: &i128) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();

    if *amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    write_admin_fee(env, amount);
    events::admin_free::set_admin_fee(env, &admin, amount);

    Ok(())
}

pub fn withdraw_admin_fee(env: &Env) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();

    let admin_balance = read_admin_balance(env);
    if admin_balance <= 0 {
        return Err(Error::AdminBalanceNotAvailable);
    }

    let contract_balance = read_contract_balance(env);
    if contract_balance < admin_balance {
        return Err(Error::InsufficientBalance);
    }

    token_transfer(env, &env.current_contract_address(), &admin, &admin_balance)?;

    write_admin_balance(env, &0_i128);

    events::admin_free::withdraw_admin_fee(env, &admin, &admin_balance);

    Ok(())
}
