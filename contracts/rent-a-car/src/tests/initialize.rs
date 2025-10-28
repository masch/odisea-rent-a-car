use soroban_sdk::{vec, IntoVal, Symbol};

use crate::tests::config::{contract::ContractTest, utils::get_contract_events};

// stellar contract deploy --wasm target/wasm32v1-none/release/rent_a_car.optimized.wasm --source admin --network testnet -- --admin GCSGBKCJER5G5GHDG7BAJM2UNDBQA6QQHQJ34CATW6LNGCGU777H6S2E --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

#[test]
pub fn test_initialize() {
    let ContractTest {
        env,
        contract,
        admin,
        token,
        ..
    } = ContractTest::setup();
    let contract_events = get_contract_events(&env, &contract.address);

    let contract_admin = contract.get_admin();

    assert_eq!(admin, contract_admin);
    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                vec![&env, *Symbol::new(&env, "contract_initialized").as_val(),],
                (admin.clone(), token.0.address.clone()).into_val(&env)
            )
        ]
    );
}
