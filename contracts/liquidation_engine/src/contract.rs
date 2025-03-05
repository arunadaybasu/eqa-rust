use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, Uint128, StdResult};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::COLLATERAL;
use cw_storage_plus::Item;

use crate::LiquidationStatusResponse;

const MIN_THRESHOLD: Item<u64> = Item::new("min_threshold");

pub fn initialize(
    deps: DepsMut,
    info: MessageInfo,
    threshold_ratio: u64,
) -> Result<Response, ContractError> {
    if threshold_ratio < 100 {
        return Err(ContractError::CustomError { 
            msg: "Threshold ratio must be at least 100%".to_string() 
        });
    }
    
    MIN_THRESHOLD.save(deps.storage, &threshold_ratio)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize")
        .add_attribute("admin", info.sender)
        .add_attribute("threshold_ratio", threshold_ratio.to_string()))
}

pub fn execute_check_liquidation(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    eqa_supply: Uint128,
    price: Uint128,
) -> Result<Response, ContractError> {
    let collateral = COLLATERAL.load(deps.storage)?;
    let threshold = MIN_THRESHOLD.load(deps.storage)?;
    
    let total_collateral_value = collateral.total_locked;
    let required_collateral = eqa_supply.checked_mul(Uint128::from(threshold))?.checked_div(Uint128::from(100u64))?;
    
    if total_collateral_value < required_collateral {
        // In a real implementation, this would trigger actual liquidation logic
        return Err(ContractError::InsufficientCollateral {});
    }
    
    Ok(Response::new()
        .add_attribute("action", "check_liquidation")
        .add_attribute("status", "solvent")
        .add_attribute("collateral_ratio", 
            format!("{}%", total_collateral_value.checked_mul(Uint128::from(100u64))?.checked_div(eqa_supply)?)))
}

pub fn query_liquidation_status(
    deps: Deps,
    eqa_supply: Uint128,
    price: Uint128,
) -> StdResult<LiquidationStatusResponse> {
    let collateral = COLLATERAL.load(deps.storage)?;
    let threshold = MIN_THRESHOLD.load(deps.storage)?;
    
    let total_collateral_value = collateral.total_locked;
    let backed_value = eqa_supply * price / Uint128::from(1_000_000u64); // assuming 6 decimal places
    
    // Calculate current ratio
    let current_ratio = if !eqa_supply.is_zero() {
        ((total_collateral_value * Uint128::from(100u64)) / eqa_supply).u128() as u64
    } else {
        0u64
    };
    
    Ok(LiquidationStatusResponse {
        is_solvent: total_collateral_value >= backed_value,
        current_ratio,
        required_ratio: threshold,
        collateral_value: total_collateral_value,
        backed_value,
    })
}
