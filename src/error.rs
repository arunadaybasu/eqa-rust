use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid funds")]
    InvalidFunds {},

    #[error("Invalid token")]
    InvalidToken {},

    #[error("Invalid price")]
    InvalidPrice {},

    #[error("Invalid amount")]
    InvalidAmount {},

    #[error("Insufficient collateral: required {required}, available {available}")]
    InsufficientCollateral { required: String, available: String },
    
    #[error("Collateralization below minimum threshold")]
    CollateralizationTooLow {},
    
    #[error("Operation not supported in the current market conditions")]
    UnsupportedMarketCondition {},
    
    #[error("Custom error: {msg}")]
    CustomError { msg: String },
}
