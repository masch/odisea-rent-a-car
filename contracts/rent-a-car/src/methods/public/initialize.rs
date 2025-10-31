use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::{has_admin, write_admin, write_admin_fee},
        token::write_token,
        types::error::Error,
    },
};

pub fn initialize(
    env: &Env,
    admin: &Address,
    token: &Address,
    admin_fee: &i128,
) -> Result<(), Error> {
    if admin == token {
        return Err(Error::AdminTokenConflict);
    }

    if has_admin(&env) {
        return Err(Error::ContractInitialized);
    }

    write_admin(env, &admin);
    write_token(env, &token);
    write_admin_fee(env, &admin_fee);

    events::contract::contract_initialized(env, admin, token, admin_fee);

    Ok(())
}
