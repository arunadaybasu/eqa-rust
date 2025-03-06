use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::COLLATERAL;

mod contract;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender))
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
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCollateralInfo {} => to_json_binary(&query_collateral_info(deps)?),
    }
}

fn query_collateral_info(deps: Deps) -> StdResult<CollateralResponse> {
    let collateral = COLLATERAL.load(deps.storage)?;
    
    Ok(CollateralResponse {
        usdc_axelar: collateral.usdc_axelar,
        usdc_noble: collateral.usdc_noble,
        total_locked: collateral.total_locked,
    })
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateCollateral { usdc_axelar: Uint128, usdc_noble: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCollateralInfo {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralResponse {
    pub usdc_axelar: Uint128,
    pub usdc_noble: Uint128,
    pub total_locked: Uint128,
}
