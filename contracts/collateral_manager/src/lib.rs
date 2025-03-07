use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    Addr,
};
use equilibria_smart_contracts::error::ContractError;

mod contract;
mod cw20_handler;
mod cross_chain;
mod state;

use crate::state::REGISTRY_ADDRESS;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Save registry address
    REGISTRY_ADDRESS.save(deps.storage, &msg.registry_address)?;
    
    // Initialize the contract
    contract::initialize(deps, info, msg.admin)?;
    
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("registry_address", msg.registry_address))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateCollateral { usdc_axelar, usdc_noble } => 
            contract::execute_update_collateral(deps, env, info, usdc_axelar, usdc_noble),
        
        ExecuteMsg::ReceiveTokens { token_addr, amount } => {
            let registry = REGISTRY_ADDRESS.load(deps.storage)?;
            cw20_handler::receive_tokens(deps, env, &registry, token_addr, info.sender.to_string(), amount)
        },
        
        ExecuteMsg::SendTokens { token_addr, recipient, amount } => {
            let registry = REGISTRY_ADDRESS.load(deps.storage)?;
            cw20_handler::send_tokens(deps, env, &registry, token_addr, recipient, amount)
        },
        
        ExecuteMsg::UpdateRegistry { new_registry_address } => {
            let admin = crate::state::ADMIN.load(deps.storage)?;
            if info.sender != admin {
                return Err(ContractError::Unauthorized {});
            }
            
            REGISTRY_ADDRESS.save(deps.storage, &new_registry_address)?;
            
            Ok(Response::new()
                .add_attribute("action", "update_registry")
                .add_attribute("new_registry_address", new_registry_address))
        },
        
        ExecuteMsg::SendViaAxelar { destination_chain, destination_address, amount } => 
            cross_chain::send_via_axelar(deps, info, destination_chain, destination_address, amount),
            
        ExecuteMsg::SendViaNoble { recipient_chain, recipient, amount } => 
            cross_chain::send_via_noble(deps, info, recipient_chain, recipient, amount),
            
        ExecuteMsg::HandleAxelarMessage { source_chain, source_address, payload } => 
            cross_chain::handle_axelar_message(deps, info, source_chain, source_address, payload),
            
        ExecuteMsg::HandleNobleMessage { source_chain, sender, payload } => 
            cross_chain::handle_noble_message(deps, info, source_chain, sender, payload),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCollateralInfo {} => 
            to_json_binary(&contract::query_collateral_info(deps)?),
        QueryMsg::GetTokenBalance { token_addr, account } => 
            to_json_binary(&cw20_handler::query_token_balance(deps, token_addr, account)?),
        QueryMsg::GetRegistryAddress {} => {
            let registry = REGISTRY_ADDRESS.load(deps.storage)?;
            to_json_binary(&RegistryResponse { address: registry })
        },
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
    pub registry_address: String,
    pub register_cross_chain: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Original functionality
    UpdateCollateral { usdc_axelar: Uint128, usdc_noble: Uint128 },
    
    // CW20 token operations
    ReceiveTokens { token_addr: String, amount: Uint128 },
    SendTokens { token_addr: String, recipient: String, amount: Uint128 },
    
    // Registry management
    UpdateRegistry { new_registry_address: String },
    
    // Cross-chain operations
    SendViaAxelar { destination_chain: String, destination_address: String, amount: Uint128 },
    SendViaNoble { recipient_chain: String, recipient: String, amount: Uint128 },
    
    // Cross-chain message handlers
    HandleAxelarMessage { source_chain: String, source_address: String, payload: Binary },
    HandleNobleMessage { source_chain: String, sender: String, payload: Binary },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCollateralInfo {},
    GetTokenBalance { token_addr: String, account: String },
    GetRegistryAddress {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralResponse {
    pub usdc_axelar: Uint128,
    pub usdc_noble: Uint128,
    pub total_locked: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegistryResponse {
    pub address: String,
}
