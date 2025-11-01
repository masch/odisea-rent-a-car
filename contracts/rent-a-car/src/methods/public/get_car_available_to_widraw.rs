use soroban_sdk::{Address, Env};

use crate::storage::{car::read_car, types::error::Error};

pub fn get_car_available_to_widraw(env: &Env, owner: &Address) -> Result<i128, Error> {
    let car = read_car(env, &owner)?;

    Ok(car.available_to_withdraw)
}
