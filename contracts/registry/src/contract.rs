use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, Addr, Order, StdResult};
use equilibria_smart_contracts::error::ContractError;

use crate::{ContractAddressResponse, AllContractsResponse};
use crate::state::{ADMIN, CONTRACTS};

pub fn initialize(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<Addr>,
) -> Result<Response, ContractError> {
    let admin_addr = admin.unwrap_or_else(|| info.sender.clone());
    ADMIN.save(deps.storage, &admin_addr)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize")
        .add_attribute("admin", admin_addr.to_string()))
}

pub fn execute_set_contract_address(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    address: String,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }
    
    // Validate address
    deps.api.addr_validate(&address)?;
    
    // Store contract address
    CONTRACTS.save(deps.storage, &name, &address)?;
    
    Ok(Response::new()
        .add_attribute("action", "set_contract_address")
        .add_attribute("name", name)
        .add_attribute("address", address))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Option<Addr>,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }
    
    if let Some(admin) = new_admin {
        ADMIN.save(deps.storage, &admin)?;
        
        Ok(Response::new()
            .add_attribute("action", "update_config")
            .add_attribute("new_admin", admin.to_string()))
    } else {
        Ok(Response::new()
            .add_attribute("action", "update_config")
            .add_attribute("admin", "unchanged"))
    }
}

pub fn query_contract_address(
    deps: Deps,
    name: String,
) -> StdResult<ContractAddressResponse> {
    let address = CONTRACTS.load(deps.storage, &name)?;
    
    Ok(ContractAddressResponse {
        name,
        address,
    })
}

pub fn query_all_contracts(
    deps: Deps,
) -> StdResult<AllContractsResponse> {
    let contracts: Vec<ContractAddressResponse> = CONTRACTS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (name, address) = item?;
            Ok(ContractAddressResponse {
                name: name.to_string(),
                address,
            })
        })
        .collect::<StdResult<_>>()?;
    
    Ok(AllContractsResponse { contracts })
}
