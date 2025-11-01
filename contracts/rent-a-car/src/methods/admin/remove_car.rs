use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::public::get_car_status::get_car_status,
    storage::{
        admin::read_admin,
        car::delete_car,
        types::{car_status::CarStatus, error::Error},
    },
};

pub fn remove_car(env: &Env, owner: &Address) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();

    let car_status = get_car_status(env, owner)?;
    if car_status != CarStatus::Available {
        return Err(Error::CarNotAvailable);
    }

    delete_car(env, owner);

    events::remove_car::car_removed(env, owner);

    Ok(())
}
