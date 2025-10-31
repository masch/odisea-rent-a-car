use soroban_sdk::{Address, Env};

use crate::storage::types::{car_status::CarStatus, error::Error};

pub trait RentACarContractTrait {
    fn __constructor(env: &Env, admin: Address, token: Address) -> Result<(), Error>;
    fn set_admin_fee(env: &Env, admin_fee: i128) -> Result<(), Error>;
    fn add_car(env: &Env, owner: Address, price_per_day: i128) -> Result<(), Error>;
    fn get_car_status(env: &Env, owner: Address) -> Result<CarStatus, Error>;
    fn rental(
        env: &Env,
        renter: Address,
        owner: Address,
        total_days_to_rent: u32,
        amount: i128,
    ) -> Result<(), Error>;
    fn remove_car(env: &Env, owner: Address) -> Result<(), Error>;
    fn payout_owner(env: &Env, owner: Address, amount: i128) -> Result<(), Error>;
}
