use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, Decimal, StdResult};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::{TOKEN_STATE, BALANCES};

pub fn calculate_dynamic_fee(market_price: Decimal) -> Decimal {
    let deviation = (market_price - Decimal::one()).abs();
    if deviation > Decimal::percent(1) {
        Decimal::percent(5) // 5% fee when EQA deviates more than 1%
    } else {
        Decimal::percent(1) // Default 1% fee
    }
}

pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    market_price: Decimal, // Price feed input
) -> Result<Response, ContractError> {
    let fee = calculate_dynamic_fee(market_price);
    let fee_amount = amount * fee;
    let final_amount = amount - fee_amount;
    
    BALANCES.update(deps.storage, &info.sender, |bal| -> StdResult<_> {
        Ok(bal.unwrap_or_default() + final_amount)
    })?;
    
    // Update total supply
    TOKEN_STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.total_supply += final_amount;
        Ok(state)
    })?;
    
    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minted", final_amount.to_string())
        .add_attribute("fee", fee_amount.to_string()))
}

pub fn execute_redeem(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    market_price: Decimal, // Price feed input
) -> Result<Response, ContractError> {
    let fee = calculate_dynamic_fee(market_price);
    let fee_amount = amount * fee;
    let final_amount = amount - fee_amount;
    
    let balance = BALANCES.load(deps.storage, &info.sender)?;
    if balance < amount {
        return Err(ContractError::CustomError {
            msg: "Insufficient funds".to_string(),
        });
    }
    
    BALANCES.save(deps.storage, &info.sender, &(balance - amount))?;
    
    // Update total supply
    TOKEN_STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.total_supply -= amount;
        Ok(state)
    })?;
    
    Ok(Response::new()
        .add_attribute("action", "redeem")
        .add_attribute("redeemed", final_amount.to_string())
        .add_attribute("fee", fee_amount.to_string()))
}
