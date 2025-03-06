use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, 
    Uint128, Addr, QueryRequest, WasmQuery
};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::oracle::{OracleQueryMsg, PriceResponse};

mod contract;
mod state;

use crate::state::{Config, CONFIG};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin: info.sender.clone(),
        oracle_address: msg.oracle_address,
        threshold_ratio: msg.threshold_ratio,
        liquidation_fee: msg.liquidation_fee.unwrap_or(5), // Default 5%
        is_active: true,
    };
    
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender)
        .add_attribute("threshold_ratio", msg.threshold_ratio.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CheckAndLiquidate { eqa_supply } => {
            // Get price from oracle if configured
            let config = CONFIG.load(deps.storage)?;
            let price = match config.oracle_address {
                Some(oracle_addr) => query_oracle_price(deps.as_ref(), oracle_addr)?,
                None => Uint128::new(1_000_000), // Default price (1.0 with 6 decimals)
            };
            
            contract::execute_check_liquidation(deps, env, info, eqa_supply, price)
        },
        ExecuteMsg::UpdateConfig { oracle_address, threshold_ratio, liquidation_fee, is_active } => 
            contract::execute_update_config(deps, env, info, oracle_address, threshold_ratio, liquidation_fee, is_active),
    }
}

fn query_oracle_price(deps: Deps, oracle_addr: Addr) -> Result<Uint128, ContractError> {
    // Query the oracle for current price
    let query_msg = OracleQueryMsg::GetPrice {
        denom: "eqa".to_string(),
    };
    
    let query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: oracle_addr.to_string(),
        msg: to_json_binary(&query_msg)?,
    });
    
    let price_response: PriceResponse = deps.querier.query(&query)
        .map_err(|e| ContractError::OracleError { 
            msg: format!("Failed to query oracle: {}", e) 
        })?;
    
    // Convert Decimal to Uint128 (assuming 6 decimal places)
    let price_u128 = (price_response.price.atomics().u128() * 1_000_000u128) 
        / 10u128.pow(price_response.price.decimal_places() as u32);
    
    Ok(Uint128::new(price_u128))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLiquidationStatus { eqa_supply } => {
            // Get price from oracle if configured
            let config = CONFIG.load(deps.storage)?;
            let price = match config.oracle_address {
                Some(oracle_addr) => query_oracle_price(deps, oracle_addr).unwrap_or(Uint128::new(1_000_000)),
                None => Uint128::new(1_000_000), // Default price (1.0 with 6 decimals)
            };
            
            to_json_binary(&contract::query_liquidation_status(deps, eqa_supply, price)?)
        },
        QueryMsg::GetConfig {} => 
            to_json_binary(&contract::query_config(deps)?),
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Message types remain the same
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub threshold_ratio: u64,   // Minimum collateralization ratio (e.g. 110%)
    pub liquidation_fee: Option<u64>,  // Fee charged during liquidation (e.g. 5%)
    pub oracle_address: Option<Addr>, // Optional oracle address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CheckAndLiquidate { eqa_supply: Uint128 },
    UpdateConfig { 
        oracle_address: Option<Addr>, 
        threshold_ratio: Option<u64>,
        liquidation_fee: Option<u64>,
        is_active: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetLiquidationStatus { eqa_supply: Uint128 },
    GetConfig {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidationStatusResponse {
    pub is_solvent: bool,
    pub current_ratio: u64,
    pub required_ratio: u64,
    pub collateral_value: Uint128,
    pub backed_value: Uint128,
    pub price: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub admin: Addr,
    pub oracle_address: Option<Addr>,
    pub threshold_ratio: u64,
    pub liquidation_fee: u64,
    pub is_active: bool,
}
