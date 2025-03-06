use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, Uint128, Decimal, StdResult};
use equilibria_smart_contracts::error::ContractError;
use cw_storage_plus::Item;

use crate::ArbitrageOpportunityResponse;

const REWARD_PERCENTAGE: Item<Decimal> = Item::new("reward_percentage");
const PEG_TARGET: Decimal = Decimal::one(); // Target price of 1.0

pub fn initialize(
    deps: DepsMut,
    info: MessageInfo,
    reward_percentage: Decimal,
) -> Result<Response, ContractError> {
    if reward_percentage > Decimal::percent(50) {
        return Err(ContractError::CustomError { 
            msg: "Reward percentage cannot exceed 50%".to_string() 
        });
    }
    
    REWARD_PERCENTAGE.save(deps.storage, &reward_percentage)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize")
        .add_attribute("admin", info.sender)
        .add_attribute("reward_percentage", reward_percentage.to_string()))
}

pub fn execute_arbitrage(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    target_price: Decimal,
) -> Result<Response, ContractError> {
    let reward_percentage = REWARD_PERCENTAGE.load(deps.storage)?;
    
    // Check if the target price moves price toward peg
    if (target_price > PEG_TARGET && PEG_TARGET > target_price) || 
       (target_price < PEG_TARGET && PEG_TARGET < target_price) {
        return Err(ContractError::InvalidPrice {});
    }
    
    // Calculate deviation from peg
    let deviation = if target_price > PEG_TARGET {
        target_price - PEG_TARGET
    } else {
        PEG_TARGET - target_price
    };
    
    // Simple reward calculation: reward = amount * deviation * reward_percentage 
    let reward_amount = amount.u128() as u64 * (deviation * reward_percentage).atomics().u128() as u64 / 1_000_000u64;
    
    // In a real implementation, this would transfer tokens to the arbitrageur
    
    Ok(Response::new()
        .add_attribute("action", "arbitrage")
        .add_attribute("trader", info.sender)
        .add_attribute("amount", amount)
        .add_attribute("reward", reward_amount.to_string())
        .add_attribute("target_price", target_price.to_string()))
}

pub fn query_arbitrage_opportunity(
    deps: Deps,
    current_price: Decimal,
) -> StdResult<ArbitrageOpportunityResponse> {
    let reward_percentage = REWARD_PERCENTAGE.load(deps.storage)?;
    
    // Calculate deviation from peg
    let deviation = if current_price > PEG_TARGET {
        current_price - PEG_TARGET
    } else {
        PEG_TARGET - current_price
    };
    
    let opportunity_exists = deviation > Decimal::percent(1); // 1% deviation
    
    // For simplicity, we're setting a fixed optimal trade size
    // In a real implementation, this would be calculated based on market depth
    let optimal_trade_size = Uint128::from(100_000u128);
    
    // Expected profit calculation
    let expected_profit = deviation * reward_percentage;
    
    // Determine direction
    let direction = if current_price > PEG_TARGET {
        "sell"
    } else {
        "buy"
    }.to_string();
    
    Ok(ArbitrageOpportunityResponse {
        opportunity_exists,
        optimal_trade_size,
        expected_profit,
        direction,
    })
}
