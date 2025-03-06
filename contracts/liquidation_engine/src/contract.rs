use cosmwasm_std::{
    DepsMut, Deps, Env, MessageInfo, Response, Uint128, StdResult,
    Addr, WasmQuery, QueryRequest, to_json_binary as to_binary, Decimal
};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::COLLATERAL;
use cw_storage_plus::Item;

use crate::state::{CONFIG, Config};
use crate::{LiquidationStatusResponse, ConfigResponse};

// Oracle query types
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
struct OracleQuery {
    get_price: GetPrice,
}

#[derive(serde::Serialize)]
struct GetPrice {
    denom: String,
}

#[derive(serde::Deserialize)]
struct PriceResponse {
    denom: String,
    price: Decimal,
    last_updated: u64,
}

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

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    oracle_address: Option<Addr>,
    threshold_ratio: Option<u64>,
    liquidation_fee: Option<u64>,
    is_active: Option<bool>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    
    // Check if the sender is the admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update config values if provided
    if let Some(oracle) = oracle_address {
        config.oracle_address = Some(oracle);
    }
    
    if let Some(ratio) = threshold_ratio {
        if ratio < 100 {
            return Err(ContractError::CustomError { 
                msg: "Threshold ratio must be at least 100%".to_string() 
            });
        }
        config.threshold_ratio = ratio;
    }
    
    if let Some(fee) = liquidation_fee {
        if fee > 20 {
            return Err(ContractError::CustomError { 
                msg: "Liquidation fee cannot exceed 20%".to_string() 
            });
        }
        config.liquidation_fee = fee;
    }
    
    if let Some(active) = is_active {
        config.is_active = active;
    }
    
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("admin", info.sender))
}

// Fix QueryConfig function which was missing
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    
    Ok(ConfigResponse {
        admin: config.admin,
        oracle_address: config.oracle_address,
        threshold_ratio: config.threshold_ratio,
        liquidation_fee: config.liquidation_fee,
        is_active: config.is_active,
    })
}
