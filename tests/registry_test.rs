#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        from_json, Addr,
    };
    use registry::{
        InstantiateMsg, ExecuteMsg, QueryMsg, ContractAddressResponse, AllContractsResponse,
    };
    
    #[test]
    fn test_registry_operations() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Initialize registry
        let info = mock_info("admin", &[]);
        let msg = InstantiateMsg {
            admin: None, // Default to sender
        };
        
        let res = registry::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(2, res.attributes.len());
        
        // Set contract address
        let msg = ExecuteMsg::SetContractAddress { 
            name: "axelar_usdc".to_string(), 
            address: "terra1axelar_usdc_address".to_string() 
        };
        
        let res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "name" && attr.value == "axelar_usdc"));
        
        // Set another contract address
        let msg = ExecuteMsg::SetContractAddress { 
            name: "noble_usdc".to_string(), 
            address: "terra1noble_usdc_address".to_string() 
        };
        
        let res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "name" && attr.value == "noble_usdc"));
        
        // Query contract address
        let query_msg = QueryMsg::GetContractAddress { name: "axelar_usdc".to_string() };
        let res: ContractAddressResponse = from_json(registry::query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        
        assert_eq!(res.name, "axelar_usdc");
        assert_eq!(res.address, "terra1axelar_usdc_address");
        
        // Query all contracts
        let query_msg = QueryMsg::GetAllContracts {};
        let res: AllContractsResponse = from_json(registry::query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        
        assert_eq!(res.contracts.len(), 2);
    }
}
