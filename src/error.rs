use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error: {msg}")]
    CustomError { msg: String },
    
    #[error("Insufficient Collateral")]
    InsufficientCollateral {},
    
    #[error("Invalid Price")]
    InvalidPrice {},
    
    #[error("Operation Disabled")]
    OperationDisabled {},
}
