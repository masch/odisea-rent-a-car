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

pub fn mint_token(
    env: &soroban_sdk::Env,
    token_admin: soroban_sdk::token::StellarAssetClient<'_>,
    token_issuer: Address,
    renter: &Address,
) {
    let amount_mint = 10_000_i128;
    token_admin
        .mock_auths(&[MockAuth {
            address: &token_issuer,
            invoke: &MockAuthInvoke {
                contract: &token_admin.address,
                fn_name: "mint",
                args: (renter.clone(), amount_mint).into_val(env),
                sub_invokes: &[],
            },
        }])
        .mint(renter, &amount_mint);
}

pub fn rental(
    env: &soroban_sdk::Env,
    contract: &crate::contract::RentACarContractClient<'_>,
    token_address: &Address,
    owner: &Address,
    renter: &Address,
    total_days: u32,
    amount: i128,
) {
    contract
        .mock_auths(&[MockAuth {
            address: &renter,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "rental",
                args: (renter.clone(), owner.clone(), total_days, amount).into_val(env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &token_address,
                    fn_name: "transfer",
                    args: (renter.clone(), contract.address.clone(), amount).into_val(env),
                    sub_invokes: &[],
                }],
            },
        }])
        .rental(&renter, &owner, &total_days, &amount);
}

pub fn payout_owner(
    env: &soroban_sdk::Env,
    contract: &crate::contract::RentACarContractClient<'_>,
    owner: &Address,
    amount: i128,
) {
    contract
        .mock_auths(&[MockAuth {
            address: owner,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "payout_owner",
                args: (owner.clone(), amount).into_val(env),
                sub_invokes: &[],
            },
        }])
        .payout_owner(owner, &amount);
}

pub fn return_car(
    env: &soroban_sdk::Env,
    contract: &crate::contract::RentACarContractClient<'_>,
    renter: &Address,
    owner: &Address,
) {
    contract
        .mock_auths(&[MockAuth {
            address: renter,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "return_car",
                args: (renter.clone(), owner.clone()).into_val(env),
                sub_invokes: &[],
            },
        }])
        .return_car(renter, owner);
}
