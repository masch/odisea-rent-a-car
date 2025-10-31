use crate::{
    storage::admin::read_admin_fee,
    tests::config::{contract::ContractTest, utils::get_contract_events},
};
use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    vec, IntoVal, Symbol,
};

#[test]
pub fn test_set_admin_fee_with_admin_user_successfully() {
    let ContractTest {
        env,
        contract,
        admin,
        ..
    } = ContractTest::setup();

    let admin_fee = 666_i128;

    contract
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "set_admin_fee",
                args: (admin_fee,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_admin_fee(&admin_fee);

    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_fee(&env));
    assert_eq!(contract_admin_fee, admin_fee);
    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                vec![
                    &env,
                    *Symbol::new(&env, "set_admin_fee").as_val(),
                    admin.clone().into_val(&env),
                ],
                (admin_fee).into_val(&env)
            )
        ]
    );
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
pub fn test_set_admin_fee_with_a_non_admin_user_fails() {
    let ContractTest { env, contract, .. } = ContractTest::setup();

    let admin_fee = 666_i128;

    contract.set_admin_fee(&admin_fee);

    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_fee(&env));
    assert_eq!(contract_admin_fee, 0);
    assert_eq!(contract_events, vec![&env,]);
}

#[test]
#[should_panic(expected = " Error(Contract, #6)")]
pub fn test_set_negative_admin_fee_fails() {
    let ContractTest {
        env,
        contract,
        admin,
        ..
    } = ContractTest::setup();

    let negative_admin_fee = -1_i128;

    contract
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "set_admin_fee",
                args: (negative_admin_fee,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_admin_fee(&negative_admin_fee);
    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin_fee = env.as_contract(&contract.address, || read_admin_fee(&env));
    assert_eq!(contract_admin_fee, 0);
    assert_eq!(contract_events, vec![&env,]);
}
