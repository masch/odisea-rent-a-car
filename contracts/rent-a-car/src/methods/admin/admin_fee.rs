use crate::{
    events,
    storage::{
        admin::{read_admin, write_admin_fee},
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
