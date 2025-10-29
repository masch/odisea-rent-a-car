use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token::token_transfer,
    storage::{
        car::{read_car, write_car},
        contract_balance::{read_contract_balance, write_contract_balance},
        rental::write_rental,
        structs::rental::Rental,
        types::{car_status::CarStatus, error::Error},
    },
};

pub fn rental(
    env: &Env,
    renter: Address,
    owner: Address,
    total_days_to_rent: u32,
    amount: i128,
) -> Result<(), Error> {
    // Validations

    // 1. Validate authorization who is calling the method
    renter.require_auth();

    // 2. Validate inputs
    if amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    if total_days_to_rent == 0 {
        return Err(Error::RentalDurationCannotBeZero);
    }

    if renter == owner {
        return Err(Error::SelfRentalNotAllowed);
    }

    // 3. Validate storage state
    let mut car = read_car(env, &owner)?;

    if car.car_status != CarStatus::Available {
        return Err(Error::CarAlreadyRented);
    }

    // 2. Business Logic
    token_transfer(&env, &renter, &env.current_contract_address(), &amount)?;

    car.car_status = CarStatus::Rented;
    car.available_to_withdraw = car
        .available_to_withdraw
        .checked_add(amount)
        .ok_or(Error::MathOverflow)?;

    let rental = Rental {
        total_days_to_rent,
        amount,
    };

    let mut contract_balance = read_contract_balance(&env);
    contract_balance = contract_balance
        .checked_add(amount)
        .ok_or(Error::MathOverflow)?;

    // 3. Storage Updates
    write_contract_balance(&env, &contract_balance);
    write_car(env, &owner, &car);
    write_rental(env, &renter, &owner, &rental);

    // 4. Events
    events::rental::rented(env, renter, owner, total_days_to_rent, amount);

    // 5. Return
    Ok(())
}
