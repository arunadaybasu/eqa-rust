use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
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
    
    #[error("Oracle Error: {msg}")]
    OracleError { msg: String },
    
    #[error("Liquidation Error: {msg}")]
    LiquidationError { msg: String },
}
