use crate::{
    storage::admin::read_admin_balance,
    tests::{
        config::{contract::ContractTest, utils::get_contract_events},
        utils::{add_car, mint_token, rental, set_admin_fee},
    },
};
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    vec, Address, IntoVal, Symbol,
};

#[test]
pub fn test_withdraw_admin_fee_with_a_admin_user_with_a_rental_with_admin_fee_successfully() {
    let ContractTest {
        env,
        contract,
        admin,
        token,
        ..
    } = ContractTest::setup();
    let (_, token_admin, token_issuer) = token;

    let admin_fee = 666_i128;

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;

    set_admin_fee(&env, &contract, &admin, admin_fee);

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

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_balance(&env));
    assert_eq!(contract_admin_fee, admin_fee);

    withdraw_admin_fee(&env, &contract, &admin);

    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_balance(&env));
    assert_eq!(contract_admin_fee, 0);
    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                vec![
                    &env,
                    *Symbol::new(&env, "withdraw_admin_fee").as_val(),
                    admin.clone().into_val(&env),
                ],
                (admin_fee).into_val(&env)
            )
        ]
    );
}

#[test]
#[should_panic(expected = " Error(Contract, #16)")]
pub fn test_withdraw_admin_fee_with_a_admin_user_with_a_rental_without_admin_fee_fails() {
    let ContractTest {
        env,
        contract,
        admin,
        token,
        ..
    } = ContractTest::setup();
    let (_, token_admin, token_issuer) = token;

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;

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
    withdraw_admin_fee(&env, &contract, &admin);

    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_balance(&env));
    assert_eq!(contract_admin_fee, 0);
    assert_eq!(contract_events, vec![&env,]);
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
pub fn test_withdraw_admin_fee_with_a_non_admin_user_fails() {
    let ContractTest { env, contract, .. } = ContractTest::setup();

    contract.withdraw_admin_fee();

    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_balance(&env));
    assert_eq!(contract_admin_fee, 0);
    assert_eq!(contract_events, vec![&env,]);
}

#[test]
#[should_panic(expected = " Error(Contract, #16)")]
pub fn test_withdraw_admin_fee_with_a_admin_user_without_rental_fails() {
    let ContractTest {
        env,
        contract,
        admin,
        ..
    } = ContractTest::setup();

    withdraw_admin_fee(&env, &contract, &admin);
    let contract_events = get_contract_events(&env, &contract.address);

    let admin_balance = env.as_contract(&contract.address, || read_admin_balance(&env));
    assert_eq!(admin_balance, 0);
    assert_eq!(contract_events, vec![&env,]);
}

fn withdraw_admin_fee(
    env: &soroban_sdk::Env,
    contract: &crate::contract::RentACarContractClient<'_>,
    admin: &Address,
) {
    contract
        .mock_auths(&[MockAuth {
            address: admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "withdraw_admin_fee",
                args: ().into_val(env),
                sub_invokes: &[],
            },
        }])
        .withdraw_admin_fee();
}
