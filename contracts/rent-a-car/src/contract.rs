use crate::{
    interfaces::contract::RentACarContractTrait,
    methods::{
        admin::{
            add_car::add_car,
            admin_fee::{set_admin_fee, withdraw_admin_fee},
            remove_car::remove_car,
        },
        owner::payout::payout,
        public::{
            get_admin_fee::get_admin_fee, get_admin_fee_to_withdraw::get_admin_fee_to_withdraw,
            get_car_available_to_widraw::get_car_available_to_widraw,
            get_car_status::get_car_status, initialize::initialize,
        },
        renter::{rental::rental, return_car::return_car},
    },
    storage::types::{car_status::CarStatus, error::Error},
};
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct RentACarContract;

#[contractimpl]
impl RentACarContractTrait for RentACarContract {
    fn __constructor(env: &Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(env, &admin, &token)
    }

    fn get_admin_fee(env: &Env) -> i128 {
        get_admin_fee(env)
    }

    fn get_admin_fee_to_withdraw(env: &Env) -> i128 {
        get_admin_fee_to_withdraw(env)
    }

    fn get_car_available_to_withdraw(env: &Env, owner: Address) -> Result<i128, Error> {
        get_car_available_to_widraw(env, &owner)
    }

    fn set_admin_fee(env: &Env, admin_fee: i128) -> Result<(), Error> {
        set_admin_fee(env, &admin_fee)
    }

    fn withdraw_admin_fee(env: &Env) -> Result<(), Error> {
        withdraw_admin_fee(env)
    }

    fn add_car(env: &Env, owner: Address, price_per_day: i128) -> Result<(), Error> {
        add_car(env, &owner, price_per_day)
    }

    fn get_car_status(env: &Env, owner: Address) -> Result<CarStatus, Error> {
        get_car_status(env, &owner)
    }

    fn rental(
        env: &Env,
        renter: Address,
        owner: Address,
        total_days_to_rent: u32,
        amount: i128,
    ) -> Result<(), Error> {
        rental(env, renter, owner, total_days_to_rent, amount)
    }

    fn payout_owner(env: &Env, owner: Address, amount: i128) -> Result<(), Error> {
        payout(env, &owner, amount)
    }

    fn remove_car(env: &Env, owner: Address) -> Result<(), Error> {
        remove_car(env, &owner)
    }

    fn return_car(env: &Env, renter: Address, owner: Address) -> Result<(), Error> {
        return_car(env, &renter, &owner)
    }
}
