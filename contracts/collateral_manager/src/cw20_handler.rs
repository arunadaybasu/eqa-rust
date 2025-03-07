use cosmwasm_std::{
    to_json_binary, WasmMsg, Response, Uint128, StdResult,
    Deps, DepsMut, MessageInfo, CosmosMsg, WasmQuery, QueryRequest, Addr, Env,
};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse};
use equilibria_smart_contracts::error::ContractError;

// Supported stablecoin registry keys
pub const AXELAR_USDC_KEY: &str = "axelar_usdc";
pub const NOBLE_USDC_KEY: &str = "noble_usdc";

// Query the registry for a contract address
fn get_contract_address(
    deps: Deps,
    registry_addr: &str,
    contract_key: &str,
) -> StdResult<String> {
    let query_msg = registry::QueryMsg::GetContractAddress {
        name: contract_key.to_string(),
    };
    
    let query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: registry_addr.to_string(),
        msg: to_json_binary(&query_msg)?,
    });
    
    let response: registry::ContractAddressResponse = deps.querier.query(&query)?;
    Ok(response.address)
}

// Verify if a token address is supported by checking registry
pub fn is_supported_token(
    deps: Deps, 
    registry_addr: &str,
    token_addr: &str,
) -> StdResult<bool> {
    let axelar_addr = get_contract_address(deps, registry_addr, AXELAR_USDC_KEY)?;
    let noble_addr = get_contract_address(deps, registry_addr, NOBLE_USDC_KEY)?;
    
    Ok(token_addr == axelar_addr || token_addr == noble_addr)
}

// Get token type from address using registry
pub fn get_token_type(
    deps: Deps,
    registry_addr: &str,
    token_addr: &str,
) -> Result<&'static str, ContractError> {
    let axelar_addr = get_contract_address(deps, registry_addr, AXELAR_USDC_KEY)?;
    let noble_addr = get_contract_address(deps, registry_addr, NOBLE_USDC_KEY)?;
    
    if token_addr == axelar_addr {
        return Ok(AXELAR_USDC_KEY);
    } else if token_addr == noble_addr {
        return Ok(NOBLE_USDC_KEY);
    }
    
    Err(ContractError::CustomError { 
        msg: format!("Unsupported token address: {}", token_addr) 
    })
}

// Transfer tokens from user to contract
pub fn receive_tokens(
    deps: DepsMut,
    env: Env,
    registry_addr: &str,
    token_addr: String,
    from: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if !is_supported_token(deps.as_ref(), registry_addr, &token_addr)? {
        return Err(ContractError::CustomError { 
            msg: format!("Unsupported token address: {}", token_addr) 
        });
    }
    
    // Create TransferFrom message to pull tokens from user to this contract
    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_addr.clone(),
        msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
            owner: from.clone(),
            recipient: env.contract.address.to_string(),
            amount,
        })?,
        funds: vec![],
    });
    
    // Update collateral state based on token type
    let token_type = get_token_type(deps.as_ref(), registry_addr, &token_addr)?;
    update_collateral_state(deps, token_type, amount, true)?;
    
    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "receive_tokens")
        .add_attribute("token", token_type)
        .add_attribute("from", from)
        .add_attribute("amount", amount.to_string()))
}

// Send tokens from contract to user
pub fn send_tokens(
    deps: DepsMut,
    env: Env,
    registry_addr: &str,
    token_addr: String,
    to: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if !is_supported_token(deps.as_ref(), registry_addr, &token_addr)? {
        return Err(ContractError::CustomError { 
            msg: format!("Unsupported token address: {}", token_addr) 
        });
    }
    
    // Create Transfer message to send tokens from this contract to the user
    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_addr.clone(),
        msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
            recipient: to.clone(),
            amount,
        })?,
        funds: vec![],
    });
    
    // Update collateral state based on token type
    let token_type = get_token_type(deps.as_ref(), registry_addr, &token_addr)?;
    update_collateral_state(deps, token_type, amount, false)?;
    
    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "send_tokens")
        .add_attribute("token", token_type)
        .add_attribute("to", to)
        .add_attribute("amount", amount.to_string()))
}

// Query the balance of a specific token
pub fn query_token_balance(
    deps: Deps,
    token_addr: String,
    account: String,
) -> StdResult<Uint128> {
    let balance_query = Cw20QueryMsg::Balance { address: account };
    
    let query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: token_addr,
        msg: to_json_binary(&balance_query)?,
    });
    
    let balance_response: BalanceResponse = deps.querier.query(&query)?;
    Ok(balance_response.balance)
}

// Update the collateral state based on token type
fn update_collateral_state(
    deps: DepsMut,
    token_type: &str,
    amount: Uint128,
    is_deposit: bool,
) -> Result<(), ContractError> {
    use equilibria_smart_contracts::state::COLLATERAL;
    
    COLLATERAL.update(deps.storage, |mut state| -> Result<_, ContractError> {
        match token_type {
            AXELAR_USDC_KEY => {
                if is_deposit {
                    state.usdc_axelar += amount;
                } else {
                    if state.usdc_axelar < amount {
                        return Err(ContractError::CustomError { 
                            msg: "Insufficient Axelar USDC balance".to_string() 
                        });
                    }
                    state.usdc_axelar -= amount;
                }
            },
            NOBLE_USDC_KEY => {
                if is_deposit {
                    state.usdc_noble += amount;
                } else {
                    if state.usdc_noble < amount {
                        return Err(ContractError::CustomError { 
                            msg: "Insufficient Noble USDC balance".to_string() 
                        });
                    }
                    state.usdc_noble -= amount;
                }
            },
            _ => return Err(ContractError::CustomError { 
                msg: format!("Unsupported token type: {}", token_type) 
            }),
        }
        
        // Update total locked amount
        state.total_locked = state.usdc_axelar + state.usdc_noble;
        Ok(state)
    })?;
    
    Ok(())
}
