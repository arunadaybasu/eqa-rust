use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    WasmMsg, Decimal,
};
use cw20::{Cw20ExecuteMsg};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::oracle::{calculate_dynamic_fee};

use crate::state::{TOKEN_INFO, TOKEN_SUPPLY, MINTER, MinterData, BALANCES, TOKEN_STATE};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MinterResponse, TokenInfoResponse};

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

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Store token info
    TOKEN_INFO.save(deps.storage, &msg.token_info)?;
    
    // Set initial supply to zero
    TOKEN_SUPPLY.save(deps.storage, &Uint128::zero())?;
    
    // Set up minter if provided
    let minter = if let Some(minter_info) = msg.minter {
        Some(MinterData {
            minter: deps.api.addr_validate(&minter_info.minter)?,
            cap: minter_info.cap,
            price_feed: minter_info.price_feed,
            collateral_denom: minter_info.collateral_denom,
        })
    } else {
        None
    };
    MINTER.save(deps.storage, &minter)?;
    
    Ok(Response::default()
        .add_attribute("action", "instantiate")
        .add_attribute("name", msg.token_info.name)
        .add_attribute("symbol", msg.token_info.symbol)
        .add_attribute("decimals", msg.token_info.decimals.to_string())
        .add_attribute("initial_supply", Uint128::zero().to_string()))
}

pub fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    market_price: Decimal,
) -> Result<Response, ContractError> {
    // Check if minter is authorized
    let minter = MINTER.load(deps.storage)?;
    if let Some(minter_data) = minter {
        if info.sender != minter_data.minter {
            return Err(ContractError::Unauthorized {});
        }
        
        // Check if we've reached cap
        let current_supply = TOKEN_SUPPLY.load(deps.storage)?;
        if let Some(cap) = minter_data.cap {
            if current_supply + amount > cap {
                return Err(ContractError::CustomError { 
                    msg: "Mint would exceed cap".to_string() 
                });
            }
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    
    // Calculate dynamic fee based on market price
    let fee = calculate_dynamic_fee(market_price, None)?;
    let fee_amount = amount * fee;
    let mint_amount = amount - fee_amount;
    
    // Update supply
    TOKEN_SUPPLY.update(deps.storage, |supply| -> StdResult<_> {
        Ok(supply + mint_amount)
    })?;
    
    // Validate recipient address
    let recipient_addr = deps.api.addr_validate(&recipient)?;
    
    // Create transfer messages
    let transfer_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: recipient.clone(),
            amount: mint_amount,
        })?,
        funds: vec![],
    };
    
    // Return success response
    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", recipient)
        .add_attribute("amount", mint_amount.to_string())
        .add_attribute("fee", fee_amount.to_string())
        .add_message(transfer_msg))
}

pub fn execute_redeem(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    market_price: Decimal, // Price feed input
) -> Result<Response, ContractError> {
    // Calculate dynamic fee
    let fee = calculate_dynamic_fee(market_price, None)?;
    let fee_amount = amount * fee;
    let final_amount = amount - fee_amount;
    
    // Check user balance
    let balance = BALANCES.load(deps.storage, &info.sender)?;
    if balance < amount {
        return Err(ContractError::CustomError {
            msg: "Insufficient funds".to_string(),
        });
    }
    
    // Update user balance
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

// Query token info
pub fn query_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let token_info = TOKEN_INFO.load(deps.storage)?;
    let total_supply = TOKEN_SUPPLY.load(deps.storage)?;
    
    Ok(TokenInfoResponse {
        name: token_info.name,
        symbol: token_info.symbol,
        decimals: token_info.decimals,
        total_supply,
    })
}

// Query minter info
pub fn query_minter(deps: Deps) -> StdResult<MinterResponse> {
    let minter = MINTER.load(deps.storage)?;
    
    match minter {
        Some(minter_data) => Ok(MinterResponse {
            minter: minter_data.minter.to_string(),
            cap: minter_data.cap,
            price_feed: minter_data.price_feed,
            collateral_denom: minter_data.collateral_denom,
        }),
        None => Ok(MinterResponse {
            minter: "".to_string(),
            cap: None,
            price_feed: None,
            collateral_denom: None,
        }),
    }
}
