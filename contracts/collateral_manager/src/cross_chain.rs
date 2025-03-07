use cosmwasm_std::{
    to_json_binary, WasmMsg, Response, Uint128, Binary,
    Deps, DepsMut, MessageInfo, CosmosMsg, SubMsg, StdResult,
};
use equilibria_smart_contracts::error::ContractError;

// Replace these placeholder addresses with actual gateway addresses in production:

// Axelar Gateway contract address
pub const AXELAR_GATEWAY: &str = "axelar_gateway_address"; // Replace with actual Axelar Gateway contract address

// Noble Gateway contract address
pub const NOBLE_GATEWAY: &str = "noble_gateway_address"; // Replace with actual Noble Gateway contract address

// Fees for cross-chain operations
pub const AXELAR_FEE: Uint128 = Uint128::new(1_000_000); // 1 USDC
pub const NOBLE_FEE: Uint128 = Uint128::new(500_000);   // 0.5 USDC

// Interface to Axelar Gateway
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum AxelarGatewayMsg {
    SendToken { 
        destination_chain: String,
        destination_address: String,
        symbol: String, 
        amount: Uint128 
    },
}

// Interface to Noble Gateway
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum NobleGatewayMsg {
    CrossChainTransfer { 
        recipient_chain: String,
        recipient: String,
        denom: String, 
        amount: Uint128 
    },
}

// Send tokens via Axelar to another chain
pub fn send_via_axelar(
    deps: DepsMut,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Verify funds sent cover the fee
    let sent_funds = info.funds.iter().find(|c| c.denom == "uluna").ok_or_else(|| {
        ContractError::CustomError { 
            msg: "No Luna sent to cover cross-chain fee".to_string() 
        }
    })?;
    
    if sent_funds.amount < AXELAR_FEE {
        return Err(ContractError::CustomError { 
            msg: format!("Insufficient fee: sent {}, required {}", sent_funds.amount, AXELAR_FEE) 
        });
    }
    
    // First transfer from our cw20 contract to Axelar gateway
    use crate::cw20_handler::{AXELAR_USDC_ADDR, receive_tokens};
    
    // Pull tokens from user to this contract
    let receive_response = receive_tokens(
        deps, 
        AXELAR_USDC_ADDR.to_string(), 
        info.sender.to_string(),
        amount
    )?;
    
    // Now create axelar cross-chain transfer message
    let axelar_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: AXELAR_GATEWAY.to_string(),
        msg: to_json_binary(&AxelarGatewayMsg::SendToken {
            destination_chain,
            destination_address,
            symbol: "USDC".to_string(),
            amount,
        })?,
        funds: vec![],
    });
    
    Ok(receive_response
        .add_message(axelar_msg)
        .add_attribute("action", "cross_chain_send_axelar")
        .add_attribute("amount", amount)
        .add_attribute("fee", AXELAR_FEE))
}

// Send tokens via Noble to another chain
pub fn send_via_noble(
    deps: DepsMut,
    info: MessageInfo,
    recipient_chain: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Verify funds sent cover the fee
    let sent_funds = info.funds.iter().find(|c| c.denom == "unble").ok_or_else(|| {
        ContractError::CustomError { 
            msg: "No Noble token sent to cover cross-chain fee".to_string() 
        }
    })?;
    
    if sent_funds.amount < NOBLE_FEE {
        return Err(ContractError::CustomError { 
            msg: format!("Insufficient fee: sent {}, required {}", sent_funds.amount, NOBLE_FEE) 
        });
    }
    
    // First transfer from our cw20 contract to Noble gateway
    use crate::cw20_handler::{NOBLE_USDC_ADDR, receive_tokens};
    
    // Pull tokens from user to this contract
    let receive_response = receive_tokens(
        deps, 
        NOBLE_USDC_ADDR.to_string(), 
        info.sender.to_string(),
        amount
    )?;
    
    // Now create Noble cross-chain transfer message
    let noble_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: NOBLE_GATEWAY.to_string(),
        msg: to_json_binary(&NobleGatewayMsg::CrossChainTransfer {
            recipient_chain,
            recipient,
            denom: "usdc".to_string(),
            amount,
        })?,
        funds: vec![],
    });
    
    Ok(receive_response
        .add_message(noble_msg)
        .add_attribute("action", "cross_chain_send_noble")
        .add_attribute("amount", amount)
        .add_attribute("fee", NOBLE_FEE))
}

// Handle incoming cross-chain messages from Axelar
pub fn handle_axelar_message(
    deps: DepsMut,
    info: MessageInfo,
    source_chain: String,
    source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    // Verify message is from Axelar gateway
    if info.sender != deps.api.addr_validate(AXELAR_GATEWAY)? {
        return Err(ContractError::Unauthorized {});
    }
    
    // Parse payload (example - actual format will depend on Axelar's implementation)
    let parsed: serde_json::Value = serde_json::from_slice(&payload)?;
    
    // Example: handle a deposit
    if let Some(action) = parsed.get("action").and_then(|a| a.as_str()) {
        match action {
            "deposit" => {
                let recipient = parsed.get("recipient")
                    .and_then(|r| r.as_str())
                    .ok_or_else(|| ContractError::CustomError { 
                        msg: "Missing recipient in payload".to_string() 
                    })?;
                    
                let amount_str = parsed.get("amount")
                    .and_then(|a| a.as_str())
                    .ok_or_else(|| ContractError::CustomError { 
                        msg: "Missing amount in payload".to_string() 
                    })?;
                
                let amount = Uint128::from_str(amount_str).map_err(|_| {
                    ContractError::CustomError { 
                        msg: "Invalid amount format".to_string() 
                    }
                })?;
                
                // Credit the deposit to recipient
                // This would add USDC to our collateral pool
                use crate::cw20_handler::update_collateral_state;
                update_collateral_state(deps, "axelar_usdc", amount, true)?;
                
                return Ok(Response::new()
                    .add_attribute("action", "handle_axelar_deposit")
                    .add_attribute("recipient", recipient)
                    .add_attribute("amount", amount)
                    .add_attribute("source_chain", source_chain)
                    .add_attribute("source_address", source_address));
            },
            _ => return Err(ContractError::CustomError { 
                msg: format!("Unknown action: {}", action) 
            }),
        }
    }
    
    Err(ContractError::CustomError { 
        msg: "Invalid payload format".to_string() 
    })
}

// Handle incoming cross-chain messages from Noble
pub fn handle_noble_message(
    deps: DepsMut,
    info: MessageInfo,
    source_chain: String,
    sender: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    // Verify message is from Noble gateway
    if info.sender != deps.api.addr_validate(NOBLE_GATEWAY)? {
        return Err(ContractError::Unauthorized {});
    }
    
    // Parse payload (example - actual format will depend on Noble's implementation)
    let parsed: serde_json::Value = serde_json::from_slice(&payload)?;
    
    // Similar to Axelar handler but with Noble-specific logic
    // ...
    
    Ok(Response::new()
        .add_attribute("action", "handle_noble_message")
        .add_attribute("source_chain", source_chain)
        .add_attribute("sender", sender))
}

// Register contract callbacks for cross-chain messages
pub fn register_callbacks(deps: DepsMut) -> Result<Response, ContractError> {
    // This would register this contract with the bridges to receive callbacks
    // Implementation details would depend on Axelar and Noble's specific APIs
    
    let axelar_register_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: AXELAR_GATEWAY.to_string(),
        msg: to_json_binary(&serde_json::json!({
            "register_receiver": {
                "contract_address": env.contract.address.to_string(),
            }
        }))?,
        funds: vec![],
    });
    
    let noble_register_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: NOBLE_GATEWAY.to_string(),
        msg: to_json_binary(&serde_json::json!({
            "register_callback": {
                "callback_address": env.contract.address.to_string(),
            }
        }))?,
        funds: vec![],
    });
    
    Ok(Response::new()
        .add_message(axelar_register_msg)
        .add_message(noble_register_msg)
        .add_attribute("action", "register_cross_chain_callbacks"))
}
