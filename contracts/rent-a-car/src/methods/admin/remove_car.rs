use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::read_admin,
        car::{delete_car, has_car},
        types::error::Error,
    },
};

pub fn remove_car(env: &Env, owner: &Address) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();

    if !has_car(env, owner) {
        return Err(Error::CarNotFound);
    }

    delete_car(env, owner);

    events::remove_car::car_removed(env, owner);

    Ok(())
}
