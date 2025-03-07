use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, Uint128, Addr};
use equilibria_smart_contracts::error::ContractError;
use equilibria_smart_contracts::state::{COLLATERAL, CollateralState};
use cw_storage_plus::Item;

pub const ADMIN: Item<Addr> = Item::new("admin");

pub fn initialize(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<Addr>,
) -> Result<Response, ContractError> {
    // Set admin (defaults to sender if not provided)
    let admin_addr = admin.unwrap_or(info.sender.clone());
    ADMIN.save(deps.storage, &admin_addr)?;
    
    // Initialize empty collateral state
    let collateral_state = CollateralState {
        usdc_axelar: Uint128::zero(),
        usdc_noble: Uint128::zero(),
        total_locked: Uint128::zero(),
    };
    COLLATERAL.save(deps.storage, &collateral_state)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize")
        .add_attribute("admin", admin_addr.to_string()))
}

pub fn execute_update_collateral(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    usdc_axelar: Uint128,
    usdc_noble: Uint128,
) -> Result<Response, ContractError> {
    // Check if caller is admin
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }
    
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

pub fn query_collateral_info(deps: Deps) -> StdResult<crate::CollateralResponse> {
    let collateral = COLLATERAL.load(deps.storage)?;
    
    Ok(crate::CollateralResponse {
        usdc_axelar: collateral.usdc_axelar,
        usdc_noble: collateral.usdc_noble,
        total_locked: collateral.total_locked,
    })
}
