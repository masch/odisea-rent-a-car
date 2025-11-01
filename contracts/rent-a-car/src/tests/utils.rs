use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    Address, IntoVal,
};

pub fn set_admin_fee(
    env: &soroban_sdk::Env,
    contract: &crate::contract::RentACarContractClient<'_>,
    admin: &Address,
    admin_fee: i128,
) {
    contract
        .mock_auths(&[MockAuth {
            address: admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "set_admin_fee",
                args: (admin_fee,).into_val(env),
                sub_invokes: &[],
            },
        }])
        .set_admin_fee(&admin_fee);
}

pub fn add_car(
    env: &soroban_sdk::Env,
    contract: &crate::contract::RentACarContractClient<'_>,
    admin: &Address,
    owner: &Address,
    price_per_day: i128,
) {
    contract
        .mock_auths(&[MockAuth {
            address: admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "add_car",
                args: (owner.clone(), price_per_day).into_val(env),
                sub_invokes: &[],
            },
        }])
        .add_car(owner, &price_per_day);
}
