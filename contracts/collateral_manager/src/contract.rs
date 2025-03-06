use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::COLLATERAL;

pub fn execute_update_collateral(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    usdc_axelar: Uint128,
    usdc_noble: Uint128,
) -> Result<Response, ContractError> {
    // In a production system, you would add authorization checks here
    // For example: if info.sender != admin { return Err(ContractError::Unauthorized {}); }
    
    COLLATERAL.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.usdc_axelar = usdc_axelar;
        state.usdc_noble = usdc_noble;
        state.total_locked = usdc_axelar + usdc_noble;
        Ok(state)
    })?;
    
    Ok(Response::new()
        .add_attribute("action", "update_collateral")
        .add_attribute("usdc_axelar", usdc_axelar)
        .add_attribute("usdc_noble", usdc_noble)
        .add_attribute("total_locked", usdc_axelar + usdc_noble))
}
