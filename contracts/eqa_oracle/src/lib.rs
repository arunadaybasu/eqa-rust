use cosmwasm_std::{
    entry_point, to_json_binary as to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, 
    Addr, StdError, Decimal
};

// Use real TerraQuerier in non-test environments
#[cfg(not(test))]
use terra_cosmwasm::TerraQuerier;

// Use mock TerraQuerier in test environments
#[cfg(test)]
use crate::mock::{MockTerraQuerier as TerraQuerier};

#[cfg(test)]
mod mock;

use equilibria_smart_contracts::error::ContractError;

mod contract;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    contract::initialize(deps, info, msg.admin, msg.price_timeout)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => contract::update_admin(deps, info, new_admin),
        ExecuteMsg::UpdatePriceTimeout { new_timeout } => contract::update_price_timeout(deps, info, new_timeout),
        ExecuteMsg::RegisterAsset { denom, symbol } => contract::register_asset(deps, info, denom, symbol),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice { denom } => to_binary(&contract::query_price(deps, env, denom)?),
        QueryMsg::GetExchangeRate { base_denom, quote_denom } => 
            to_binary(&contract::query_exchange_rate(deps, env, base_denom, quote_denom)?),
        QueryMsg::GetRegisteredAssets {} => to_binary(&contract::query_registered_assets(deps)?),
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Addr,
    pub price_timeout: u64, // in seconds
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateAdmin { new_admin: Addr },
    UpdatePriceTimeout { new_timeout: u64 },
    RegisterAsset { denom: String, symbol: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPrice { denom: String },
    GetExchangeRate { base_denom: String, quote_denom: String },
    GetRegisteredAssets {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub denom: String,
    pub price: Decimal,
    pub last_updated: u64, // timestamp
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRateResponse {
    pub base_denom: String,
    pub quote_denom: String,
    pub rate: Decimal,
    pub last_updated: u64, // timestamp
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredAssetResponse {
    pub assets: Vec<RegisteredAsset>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredAsset {
    pub denom: String,
    pub symbol: String,
}
