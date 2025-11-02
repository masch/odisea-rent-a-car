use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        car::{read_car, write_car},
        rental::remove_rental,
        types::{car_status::CarStatus, error::Error},
    },
};

pub fn return_car(env: &Env, renter: &Address, owner: &Address) -> Result<(), Error> {
    renter.require_auth();

    let mut car = read_car(env, &owner)?;

    if car.car_status != CarStatus::Rented {
        return Err(Error::CarNotRented);
    }

    car.car_status = CarStatus::Available;

    write_car(&env, &owner, &car);
    remove_rental(env, &renter, &owner);

    events::return_car::car_returned(env, owner);

    Ok(())
}
