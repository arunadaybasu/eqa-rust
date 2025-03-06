use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, Decimal, StdResult, WasmQuery, QueryRequest, to_binary};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::{TOKEN_STATE, BALANCES};

// Add oracle-related imports
use crate::state::CONFIG;

// Oracle query types - these would match your oracle contract
#[derive(serde::Serialize)]
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

pub fn calculate_dynamic_fee(market_price: Decimal) -> Decimal {
    let deviation = if market_price > Decimal::one() {
        market_price - Decimal::one()
    } else {
        Decimal::one() - market_price
    };
    
    if deviation > Decimal::percent(1) {
        Decimal::percent(5) // 5% fee when EQA deviates more than 1%
    } else {
        Decimal::percent(1) // Default 1% fee
    }
}

pub fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    market_price: Option<Decimal>, // Optional price feed input
) -> Result<Response, ContractError> {
    // Get actual market price from oracle if not provided
    let actual_market_price = match market_price {
        Some(price) => price,
        None => {
            let config = CONFIG.load(deps.storage)?;
            if let Some(oracle_address) = config.oracle_address {
                // Query the oracle for the current price
                let query_msg = OracleQuery {
                    get_price: GetPrice {
                        denom: "eqa".to_string(),
                    },
                };
                
                let query = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: oracle_address.to_string(),
                    msg: to_binary(&query_msg)?,
                });
                
                let price_response: PriceResponse = deps.querier.query(&query)?;
                price_response.price
            } else {
                // Fallback to default peg if oracle not set
                Decimal::one()
            }
        }
    };

    let fee = calculate_dynamic_fee(actual_market_price);
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
        .add_attribute("fee", fee_amount.to_string())
        .add_attribute("market_price", actual_market_price.to_string()))
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
