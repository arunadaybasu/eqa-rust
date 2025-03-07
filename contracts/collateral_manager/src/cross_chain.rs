use cosmwasm_std::{
    to_json_binary, WasmMsg, Response, Uint128, Binary,
    DepsMut, MessageInfo, CosmosMsg, Env,
};
use equilibria_smart_contracts::error::ContractError;

// Axelar Gateway contract address key in registry
pub const AXELAR_GATEWAY_KEY: &str = "axelar_gateway";
pub const NOBLE_GATEWAY_KEY: &str = "noble_gateway";

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
    mut deps: DepsMut,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Load registry address
    let registry_addr = crate::state::REGISTRY_ADDRESS.load(deps.storage)?;
    
    // Get Axelar gateway address from registry
    let axelar_gateway = crate::cw20_handler::get_contract_address(
        deps.as_ref(), 
        &registry_addr, 
        AXELAR_GATEWAY_KEY
    )?;
    
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
    
    // Get token address from registry
    use crate::cw20_handler::{AXELAR_USDC_KEY, update_collateral_with_token_type};
    let token_addr = crate::cw20_handler::get_contract_address(
        deps.as_ref(), 
        &registry_addr, 
        AXELAR_USDC_KEY
    )?;
    
    // Update collateral state directly instead of using receive_tokens
    update_collateral_with_token_type(deps.branch(), AXELAR_USDC_KEY, amount, true)?;
    
    // Create transfer message - simplified version
    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_addr,
        msg: to_json_binary(&cw20::Cw20ExecuteMsg::TransferFrom {
            owner: info.sender.to_string(),
            recipient: "contract_address".to_string(), // This is simplified
            amount,
        })?,
        funds: vec![],
    });
    
    // Create axelar cross-chain transfer message
    let axelar_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: axelar_gateway,
        msg: to_json_binary(&AxelarGatewayMsg::SendToken {
            destination_chain,
            destination_address,
            symbol: "USDC".to_string(),
            amount,
        })?,
        funds: vec![],
    });
    
    Ok(Response::new()
        .add_message(transfer_msg)
        .add_message(axelar_msg)
        .add_attribute("action", "cross_chain_send_axelar")
        .add_attribute("amount", amount.to_string())
        .add_attribute("fee", AXELAR_FEE.to_string()))
}

// Send tokens via Noble to another chain - simplified for compilation
pub fn send_via_noble(
    deps: DepsMut,
    _info: MessageInfo, // Prefix with underscore to address warning
    recipient_chain: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Simplified implementation
    let registry_addr = crate::state::REGISTRY_ADDRESS.load(deps.storage)?;
    let noble_gateway = crate::cw20_handler::get_contract_address(
        deps.as_ref(), 
        &registry_addr, 
        NOBLE_GATEWAY_KEY
    )?;
    
    Ok(Response::new()
        .add_attribute("action", "cross_chain_send_noble")
        .add_attribute("gateway", noble_gateway)
        .add_attribute("amount", amount.to_string())
        .add_attribute("recipient_chain", recipient_chain)
        .add_attribute("recipient", recipient))
}

// Handle incoming cross-chain messages from Axelar
pub fn handle_axelar_message(
    _deps: DepsMut, // Prefix with underscore to address warning
    _info: MessageInfo, // Prefix with underscore to address warning
    source_chain: String,
    source_address: String,
    _payload: Binary, // Prefix with underscore to address warning
) -> Result<Response, ContractError> {
    // Simplified implementation
    Ok(Response::new()
        .add_attribute("action", "handle_axelar_message")
        .add_attribute("source_chain", source_chain)
        .add_attribute("source_address", source_address))
}

// Handle incoming cross-chain messages from Noble
pub fn handle_noble_message(
    _deps: DepsMut, // Prefix with underscore to address warning
    _info: MessageInfo, // Prefix with underscore to address warning
    source_chain: String,
    sender: String,
    _payload: Binary, // Prefix with underscore to address warning
) -> Result<Response, ContractError> {
    // Simplified implementation
    Ok(Response::new()
        .add_attribute("action", "handle_noble_message")
        .add_attribute("source_chain", source_chain)
        .add_attribute("sender", sender))
}

// Register contract callbacks for cross-chain messages
pub fn register_callbacks(
    deps: DepsMut,
    env: Env,
) -> Result<Response, ContractError> {
    // Load registry address
    let registry_addr = crate::state::REGISTRY_ADDRESS.load(deps.storage)?;
    
    let axelar_gateway = crate::cw20_handler::get_contract_address(
        deps.as_ref(), 
        &registry_addr, 
        AXELAR_GATEWAY_KEY
    )?;
    
    let noble_gateway = crate::cw20_handler::get_contract_address(
        deps.as_ref(), 
        &registry_addr, 
        NOBLE_GATEWAY_KEY
    )?;
    
    // Create register messages
    let axelar_register_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: axelar_gateway,
        msg: to_json_binary(&serde_json::json!({
            "register_receiver": {
                "contract_address": env.contract.address.to_string(),
            }
        }))?,
        funds: vec![],
    });
    
    let noble_register_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: noble_gateway,
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
