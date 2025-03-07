use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Decimal,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use equilibria_smart_contracts::error::ContractError;

mod contract;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    contract::initialize(deps, info, msg.reward_percentage)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Arbitrage { amount, target_price } => {
            contract::execute_arbitrage(deps, env, info, amount, target_price)
        }
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::ArbitrageOpportunity { current_price } => {
            let result = contract::query_arbitrage_opportunity(deps, current_price)?;
            to_json_binary(&result)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub reward_percentage: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Arbitrage { amount: Uint128, target_price: Decimal },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ArbitrageOpportunity { current_price: Decimal },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ArbitrageOpportunityResponse {
    pub opportunity_exists: bool,
    pub optimal_trade_size: Uint128,
    pub expected_profit: Decimal,
    pub direction: String,
}
