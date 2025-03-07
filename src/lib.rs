pub mod state;
pub mod error;
pub mod math;
pub mod config;
pub mod network;
pub mod oracle;
pub mod mocks;

use cosmwasm_std::{StdError, StdResult, Storage, Uint128};

pub fn calculate_collateral_ratio(
    storage: &dyn Storage, 
    debt_amount: Uint128
) -> StdResult<Uint128> {
    let state = state::COLLATERAL.load(storage)?;
    
    if debt_amount.is_zero() {
        return Err(StdError::generic_err("Debt amount cannot be zero"));
    }
    
    // Ratio = (collateral / debt) * 100 to get percentage
    let ratio = state.total_locked
        .checked_mul(Uint128::from(100u64))
        .map_err(|_| StdError::generic_err("Multiplication overflow"))?
        .checked_div(debt_amount)
        .map_err(|_| StdError::generic_err("Division by zero"))?;
    
    Ok(ratio)
}

pub fn is_properly_collateralized(
    storage: &dyn Storage,
    debt_amount: Uint128,
    min_ratio: u64,
) -> StdResult<bool> {
    let ratio = calculate_collateral_ratio(storage, debt_amount)?;
    
    Ok(ratio.u128() >= min_ratio as u128)
}
