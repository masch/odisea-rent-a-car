use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ContractInitialized = 0,
    ContractNotInitialized = 1,
    CarNotFound = 2,
    AdminTokenConflict = 3,
    RentalDurationCannotBeZero = 4,
    CarAlreadyExist = 5,
    AmountMustBePositive = 6,
    RentalNotFound = 7,
    InsufficientBalance = 8,
    BalanceNotAvailableForAmountRequested = 9,
    SelfRentalNotAllowed = 10,
    CarAlreadyRented = 11,
    TokenNotFound = 12,
    AdminNotFound = 13,
    MathOverflow = 14,
    MathUnderflow = 15,
    AdminBalanceNotAvailable = 16,
    CarNotAvailable = 17,
    CarNotRented = 18,
}
