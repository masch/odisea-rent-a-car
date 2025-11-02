use crate::{
    storage::{car::read_car, contract_balance::read_contract_balance, rental::has_rental},
    tests::{
        config::{contract::ContractTest, utils::get_contract_events},
        utils::{add_car, mint_token, rental, return_car},
    },
};
use soroban_sdk::{testutils::Address as _, vec, Address, IntoVal, Symbol};

#[test]
pub fn test_return_car_successfully() {
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

    let has_rental_contract =
        env.as_contract(&contract.address, || has_rental(&env, &renter, &owner));
    assert_eq!(has_rental_contract, true);

    return_car(&env, &contract, &renter, &owner);

    let contract_events = get_contract_events(&env, &contract.address);
    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                vec![
                    &env,
                    *Symbol::new(&env, "car_retuned").as_val(),
                    owner.clone().into_val(&env),
                ],
                ().into_val(&env)
            )
        ]
    );

    let has_rental_contract =
        env.as_contract(&contract.address, || has_rental(&env, &renter, &owner));
    assert_eq!(has_rental_contract, false);

    let car = env
        .as_contract(&contract.address, || read_car(&env, &owner))
        .unwrap(); // TODO handle error properly
    assert_eq!(car.available_to_withdraw, amount);

    let contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    assert_eq!(contract_balance, amount);
}

#[test]
#[should_panic(expected = " Error(Contract, #18)")]
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

    return_car(&env, &contract, &renter, &owner);

    // Should be fails if tries to return two times the same rent an
    return_car(&env, &contract, &renter, &owner);
}

//TODO: Add test validation
