use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::COLLATERAL;

mod contract;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    contract::initialize(deps, info, msg.threshold_ratio)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CheckAndLiquidate { eqa_supply, price } => 
            contract::execute_check_liquidation(deps, env, info, eqa_supply, price),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLiquidationStatus { eqa_supply, price } => 
            to_binary(&contract::query_liquidation_status(deps, eqa_supply, price)?),
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub threshold_ratio: u64, // Minimum collateralization ratio (e.g. 110 means 110%)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CheckAndLiquidate { eqa_supply: Uint128, price: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetLiquidationStatus { eqa_supply: Uint128, price: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidationStatusResponse {
    pub is_solvent: bool,
    pub current_ratio: u64,
    pub required_ratio: u64,
    pub collateral_value: Uint128,
    pub backed_value: Uint128,
}
