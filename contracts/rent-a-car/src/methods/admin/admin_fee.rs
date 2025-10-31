use crate::{
    events,
    storage::{
        admin::{read_admin, write_admin_fee},
        types::error::Error,
    },
};
use soroban_sdk::Env;

pub fn set_admin_fee(env: &Env, admin_fee: &u128) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();

    write_admin_fee(env, admin_fee);
    events::admin_free::set_admin_fee(env, &admin, admin_fee);

    Ok(())
}
