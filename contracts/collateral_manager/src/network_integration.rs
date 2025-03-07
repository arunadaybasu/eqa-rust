use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, StdResult};
use equilibria_smart_contracts::config::NetworkEnvironment;
use equilibria_smart_contracts::network::NetworkManager;
use equilibria_smart_contracts::error::ContractError;

/// Re-configure network environment based on current chain
pub fn auto_configure_network(
    deps: DepsMut, 
    env: &Env
) -> Result<Response, ContractError> {
    let network = NetworkManager::auto_configure_network(deps, env)?;
    
    Ok(Response::new()
        .add_attribute("action", "auto_configure_network")
        .add_attribute("network", network.to_string()))
}

/// Manually set network environment with admin authorization
pub fn set_network_environment(
    deps: DepsMut, 
    info: MessageInfo,
    network: NetworkEnvironment
) -> Result<Response, ContractError> {
    // Load admin to check authorization
    let admin = crate::state::ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update network in configuration
    equilibria_smart_contracts::config::storage::set_active_network(
        deps.storage,
        network.clone(),
    )?;
    
    Ok(Response::new()
        .add_attribute("action", "set_network_environment")
        .add_attribute("network", network.to_string()))
}

/// Query gateway address based on current network configuration
pub fn query_gateway_address(
    deps: Deps,
    gateway_type: String,
) -> StdResult<String> {
    NetworkManager::get_contract_address(deps, &format!("{}_gateway", gateway_type))
}
