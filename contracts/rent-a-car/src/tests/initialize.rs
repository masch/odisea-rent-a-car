use crate::tests::config::contract::ContractTest;

// stellar contract deploy --wasm target/wasm32v1-none/release/rent_a_car.optimized.wasm --source admin --network testnet -- --admin GCSGBKCJER5G5GHDG7BAJM2UNDBQA6QQHQJ34CATW6LNGCGU777H6S2E --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

#[test]
pub fn test_initialize() {
    let ContractTest {
        contract, admin, ..
    } = ContractTest::setup();

    let contract_admin = contract.get_admin();

    assert_eq!(admin, contract_admin);
}
