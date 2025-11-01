use crate::{
    storage::{car::read_car, contract_balance::read_contract_balance},
    tests::{
        config::{contract::ContractTest, utils::get_contract_events},
        utils::{add_car, mint_token, payout_owner, rental, return_car},
    },
};
use soroban_sdk::{testutils::Address as _, vec, Address, IntoVal, Symbol};

#[test]
pub fn test_payout_owner_returned_car_successfully() {
    let ContractTest {
        env,
        contract,
        token,
        admin,
        ..
    } = ContractTest::setup();

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;

    let (_, token_admin, token_issuer) = token;

    mint_token(&env, token_admin, token_issuer, &renter);

    add_car(&env, &contract, &admin, &owner, price_per_day);

    rental(
        &env,
        &contract,
        &token.0.address,
        &owner,
        &renter,
        total_days,
        amount,
    );

    let contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    assert_eq!(contract_balance, amount);

    return_car(&env, &contract, &renter, &owner);

    payout_owner(&env, &contract, &owner, amount);

    let contract_events = get_contract_events(&env, &contract.address);

    let car = env
        .as_contract(&contract.address, || read_car(&env, &owner))
        .unwrap(); // TODO handle error properly
    assert_eq!(car.available_to_withdraw, 0);

    let contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    assert_eq!(contract_balance, 0);

    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                vec![
                    &env,
                    *Symbol::new(&env, "payout").as_val(),
                    owner.clone().into_val(&env),
                ],
                amount.into_val(&env)
            )
        ]
    );
}

#[test]
#[should_panic(expected = " Error(Contract, #17)")]
pub fn test_payout_owner_with_rented_car_fails() {
    let ContractTest {
        env,
        contract,
        token,
        admin,
        ..
    } = ContractTest::setup();

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;

    let (_, token_admin, token_issuer) = token;

    mint_token(&env, token_admin, token_issuer, &renter);

    add_car(&env, &contract, &admin, &owner, price_per_day);

    rental(
        &env,
        &contract,
        &token.0.address,
        &owner,
        &renter,
        total_days,
        amount,
    );

    payout_owner(&env, &contract, &owner, amount);
}

//TODO: Add test validation
