use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,                    // Address of the contract administrator
    AdminFee,                 // Admin fee (e.g., fixed amount or basis points)
    AdminBalance,             // Accumulated admin balance available for withdrawal
    Token,                    // Address of the accepted payment token
    ContractBalance,          // Contract balance in the payment token
    Car(Address),             // Car associated with an owner
    Rental(Address, Address), // Rental record between renter and owner
}
